<?php
	define('HONGBAO', '14392854');
	define('YEARS', [
		2026 => [1771171200, 'cheers.b0288a5b.png'],
		2014 => [1391011200, 'cheers.c8eae588.png'],
		2002 => [1013356800, 'cheers.32198280.png']
	]);
	$debugTime = (int)($_GET['tmstmp__dbgskrt'] ?? '');
	if($debugTime && ((int)($_GET['debug'] ?? '') === 114514)) {
		$time = $debugTime;
	} else if($httpTime = (string)($_SERVER['HTTP_DATE'] ?? '')) {
		try {
			$time = \DateTime::createFromFormat('D, d M Y H:i:s T', $httpTime)->getTimestamp();
		} catch(\Throwable) {}
		$time = $time ?? time();
	} else {
		$time = time();
	}
	$year = null;
	foreach(YEARS as $yr => [$eve]) {
		$end = $eve + 86400 * 8;
		if(($eve <= $time) && ($time < $end)) {
			$year = $yr;
			break;
		}
	}

	$browserGood = (bool)preg_match('/ MSIE [56]\./', $_SERVER['HTTP_USER_AGENT'] ?? '');
?><html>
	<head>
		<meta http-equiv="Content-Type" content="text/html; charset=utf-8">
		<meta name="Author" content="Soha Jin">
		<meta name="GENERATOR" content="Microsoft FrontPage 6.0">
		<title>新年快乐！</title>
		<style>
			body { background-image: url('bg.gif'); background-repeat: repeat; text-align: center; font-family: SimSun, "宋体", serif; }
			.container { background: black; color: white; width: 480px; margin: 0 auto; padding: 0; font-size: 18px; }
			.texts { padding-left: 10px; padding-right: 10px; padding-bottom: 20px; }
			.footer { color: white; width: 480px; margin: 0 auto; padding: 0; font-size: 10px; }
		</style>
	</head>
	<body>
		<?php if(!$year): ?>
		<div class="container">
			<br />
			<p>现在是<?php echo date('Y年m月d日', $time); ?>，不在马年春节活动时间哦。</p>
			<br />
		</div>
		<?php else: ?>
		<div class="container">
			<img src="<?php echo YEARS[$year][1]; ?>" style="width: 480px;" />
			<p>
				╭★☆★☆★☆★☆★╮<br /><br />
				✿❀ 马年大吉 ❀✿<br /><br />
				╰★☆★☆★☆★☆★╯<br /><br />
			</p>
			<div class="texts">
				<p id="date">现在是<?php echo date('m月d日', $time)."，<span style=\"color: red;\">{$year}马年春节</span>"; ?>。</p>
				<?php if($year === 2026): ?>
				<p>虽然今年没有准备红包，但祝您：</p>
				<br />
				保持热爱，马不停蹄。<br />
				能量充沛，灵感不竭。<br />
				所求皆如愿，所行化坦途。<br />
				愿你万事顺遂，马到成功。<br />
				<?php elseif($year === 2014): ?>
				<p>今年没有准备红包，祝您：</p>
				<br />
				事业高端大气上档次，<br />
				生活低调奢华有内涵！<br />
				告别心塞、告别压力山大，<br />
				愿你的幸福感满得要溢出来！<br />
				<br />
				马上有钱、<br />
				马上有房、<br />
				马上有对象<br />
				<br />
				平安喜乐，万事顺遂！
				<?php elseif($year === 2002): ?>
				不当蛋白质，天天都开心 ^_^<br />
				不酱紫郁闷，不成天7456 &gt;_&lt;<br />
				电脑不当机，上网不断线 :-)<br />
				所有的帖子都有人顶，<br />
				所有的祝福都被人稀饭！<br />
				虽然偶们经常在网上冲浪灌水，<br />
				但偶对泥的祝福是真滴！<br />
				<br />
				祝各位GG都能在聊天室里遇到超级MM！<br />
				祝各位MM身边全是帅过道明寺的GG！<br />
				<br />
				偶走先，886！新年快乐，记得Q偶！<br />
				<?php if($browserGood): ?>
				<hr />
				<p style="text-align: left; font-size: 14px;">另外，过两年会有个名叫“支付宝”的平台出现在互联网上，他在十几年里会逐渐成为非常重要的互联网服务。等到了丙午马年的大年初一，别忘了用<code><?php echo HONGBAO; ?></code>领取一个红包。</p>
				<?php endif; ?>
				<?php endif; ?>
			</div>
		</div>
		<?php endif; ?>
		<div class="footer">
			Copyright &copy; <?php echo $year ?? 2026; ?> Soha Jin.<br />
			本站建议使用 IE 5 或 IE 6 浏览，建议设置 800x600 分辨率以获得最佳体验。
		</div>
	</body>
</html>