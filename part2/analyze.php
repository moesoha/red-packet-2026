<?php

$grouped = [];
$cwd = getcwd();

foreach(scandir($cwd) as $filename) {
	if(!str_ends_with($filename, '.json')) continue;
	$path = $cwd.DIRECTORY_SEPARATOR.$filename;
	if(!is_file($path)) continue;
	$data = json_decode(file_get_contents($path) ?: '', true);
	if(!$data) continue;
	
	$txt = '';
	
	$txt .= 'HB2026-Time: '.date('c', $data['time'] / 1000)."\r\n";
	$txt .= "HB2026-Sock: {$data['sock']}\r\n";
	
	if($data['macros'] ?? []) {
		foreach(['{rcpt_host}', 'j', 'v', '{daemon_name}', '{daemon_addr}', '{rcpt_mailer}', '{mail_host}', '{mail_mailer}'] as $key) {
			unset($data['macros'][$key]);
		}
		foreach($data['macros'] as $k => $v) {
			$txt .= "HB2026-Macro: $k=$v\r\n";
		}
	}
	foreach($data['header'] as [$k, $v]) {
		$txt .= "$k: $v\r\n";
	}
	$txt .= "\r\n";
	
	foreach($data['body'] as $b) {
		$txt .= base64_decode($b);
		$txt .= "\r\n";
	}
	
	$ip = trim(substr($data['sock'], 0, strrpos($data['sock'], ':')), '[]');
	$ip = inet_pton($ip);
	$grouped["\x00\x00\x00\x00"][] = $txt;
}

$output = $cwd.DIRECTORY_SEPARATOR.'OUTPUT';
if(!is_dir($output)) {
	mkdir($output);
}
foreach($grouped as $ip => $mails) {
	$file = $output.DIRECTORY_SEPARATOR.(strlen($ip) === 4 ? inet_ntop($ip) : bin2hex($ip));
	file_put_contents($file, implode("\r\n\r\n========================================\r\n\r\n", $mails));
}