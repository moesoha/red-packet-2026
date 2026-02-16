use bytes::Bytes;
use indymilter::{Callbacks, Context, EomContext, SetErrorReply, SocketInfo, Status};
use std::{env, ffi::CString, process, net::SocketAddr};
use std::collections::HashMap;
use std::net::Ipv6Addr;
use base64::Engine;
use tokio::{net::TcpListener, signal};
use ipnetwork::Ipv6Network;

lazy_static::lazy_static! {
    static ref IP_WHITELIST: Vec<Ipv6Network> = vec![
		Ipv6Network::new_checked(Ipv6Addr::new(0x2a09, 0xbac0, 0, 0, 0, 0, 0, 0), 29).unwrap(),
		Ipv6Network::new_checked(Ipv6Addr::new(0x2606, 0x54c0, 0, 0, 0, 0, 0, 0), 30).unwrap(),
		Ipv6Network::new_checked(Ipv6Addr::new(0x2a0e, 0xaa06, 0x40d, 0xbeef, 0, 0, 0, 0), 64).unwrap(),
		Ipv6Network::new_checked(Ipv6Addr::new(0x2a0e, 0xaa06, 0x40e, 0xbeef, 0, 0, 0, 0), 64).unwrap()
	];
}
const RECIPIENT_ADDR: &'static str = "get@ma26.hb.lohu.info";
const SENDER_SUFFIX: &'static str = "@hb26.foobar.ac.cn";
const HONGBAO_RESPONSE: &'static str = "Hongbao: 82460732";

#[derive(Debug, serde::Serialize)]
struct MailCtx {
	sock: Option<SocketAddr>,
	header: Vec<(String, String)>,
	body: Vec<String>,
	macros: HashMap<String, String>,
	time: i64
}

#[tokio::main]
async fn main() {
	let args = env::args().collect::<Vec<_>>();

	if args.len() != 2 {
		eprintln!("usage: {} <socket>", args[0]);
		process::exit(1);
	}

	let listener = TcpListener::bind(&args[1])
		.await
		.expect("cannot open milter socket");

	let callbacks = Callbacks::new()
		.on_connect(|ctx, _, si| Box::pin(handle_connect(ctx, si)))
		.on_header(|ctx, name, value| Box::pin(handle_header(ctx, name, value)))
		.on_body(|ctx, chunk| Box::pin(handle_body(ctx, chunk)))
		.on_eom(|ctx| Box::pin(handle_eom(ctx)))
	;

	let config = Default::default();

	indymilter::run(listener, callbacks, config, signal::ctrl_c())
		.await
		.expect("milter execution failed");
}

async fn handle_connect(ctx: &mut Context<MailCtx>, socket_info: SocketInfo) -> Status {
	ctx.data = Some(MailCtx {
		sock: match socket_info {
			SocketInfo::Inet(x) => Some(x),
			_ => None
		},
		header: Vec::new(),
		body: Vec::new(),
		macros: HashMap::new(),
		time: chrono::Utc::now().timestamp_millis()
	});

	Status::Continue
}

async fn handle_header(ctx: &mut Context<MailCtx>, name: CString, value: CString) -> Status {
	if let Some(data) = &mut ctx.data {
		data.header.push((
			String::from_utf8_lossy(name.as_bytes()).to_string(),
			String::from_utf8_lossy(value.as_bytes()).to_string()
		));
		Status::Continue
	} else {
		Status::Tempfail
	}
}

async fn handle_body(ctx: &mut Context<MailCtx>, chunk: Bytes) -> Status {
	if let Some(data) = &mut ctx.data {
		data.body.push(base64::prelude::BASE64_STANDARD.encode(chunk));
		Status::Continue
	} else {
		Status::Tempfail
	}
}

async fn handle_eom(ctx: &mut EomContext<MailCtx>) -> Status {
	let mid = ctx.macros.get(c"i")
		.and_then(|x| x.to_str().ok())
		.unwrap_or("unknown")
	;
	let data = match &mut ctx.data {
		None => return Status::Tempfail,
		Some(data) => {
			data.macros = ctx.macros.to_hash_map().iter().map(|(k, v)| (
				String::from_utf8_lossy(k.as_bytes()).to_string(),
				String::from_utf8_lossy(v.as_bytes()).to_string()
			)).collect();
			data
		}
	};
	if let Ok(json) = serde_json::to_string(data) {
		if let Err(e) = std::fs::write(format!("{}-{}.json", data.time, mid), &json) {
			eprintln!("error writing JSON: {}", e);
			println!("{} JSON: {}", mid, &json);
		}
	}

	let rcpt = ctx.macros.get(c"{rcpt_addr}")
		.and_then(|x| x.to_str().ok())
		.map(|x| x.to_lowercase())
	;
	if let Some(rcpt) = rcpt {
		if &rcpt != RECIPIENT_ADDR {
			_ =ctx.reply.set_error_reply("550", Some("5.1.1"), vec![
				"This mailbox is not found."
			]);
			return Status::Reject;
		}
	} else {
		return Status::Tempfail;
	}

	let send = ctx.macros.get(c"{mail_addr}")
		.and_then(|x| x.to_str().ok())
		.map(|x| x.to_lowercase())
	;
	if let Some(send) = send {
		if !send.ends_with(SENDER_SUFFIX) {
			_ =ctx.reply.set_error_reply("550", Some("5.7.1"), vec![
				"Sender domain is not in the whitelist."
			]);
			return Status::Reject;
		}
	} else {
		return Status::Tempfail;
	}

	let ip_pass = match data.sock {
		Some(SocketAddr::V6(a)) => IP_WHITELIST.iter()
			.any(|n| n.contains(*a.ip())),
		_ => false
	};
	if !ip_pass {
		_ =ctx.reply.set_error_reply("550", Some("5.5.0"), vec![
			"Rejected by spam filter"
		]);
		return Status::Reject;
	}

	_ =ctx.reply.set_error_reply("550", Some("5.2.2"), vec![
		"Happy New Horse Year!",
		HONGBAO_RESPONSE
	]);
	Status::Reject
}
