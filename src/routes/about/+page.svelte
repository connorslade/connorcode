<script>
	import { VERSION } from '$lib/consts';
	import Link from '$lib/Link.svelte';
</script>

<h1>About</h1>

<span id="greeting">Hello</span>, I'm Connor Slade. Welcome to my little place on the internet,
<Link href="https://connorcode.com">connorcode.com</Link> <em>version {VERSION}</em>!
<br /><br />

I'm interested in
<Link href="https://photos.connorcode.com">Photography</Link>,
<Link href="https://www.minecraft.net">Minecraft</Link>,
<Link href="https://github.connorcode.com">Software development</Link> and some Web development. The
Best programming language is
<Link href="https://www.rust-lang.org/">Rust</Link>! Like just look at the cute
<Link href="https://rustacean.dev">mascot</Link>. I use it for most of my projects. In fact, I'm
hosting this site with Rust and my own dependency free web server framework
<Link href="https://crates.io/crates/afire">afire</Link>.

<br /><br />

I started my <em>programming journey</em> around 2014 with the
<a href="https://www.arduino.cc">Arduino</a>. Then I learned how to make basic websites with HTML
and CSS, then how to make them interactive with JavaScript. After that, I got into Python and
eventually started to learn much more about Linux, Docker, different programming languages. After
trying NodeJS, Nim, Assembly, C#, Java, Kotlin, Ruby, Lua and many more I eventually settled on
<em>Rust</em> for most of my projects. <br />

<h2>Privacy Policy</h2>
Wow, you're actually reading the Privacy Policy. That's gotta be a first!
<br /><br />
I care about digital privacy, that's why I'm not running Google Analytics or something like it. This
website runs a
<em>custom open source analytics system</em> I made. You can view the
<a href="https://github.com/Basicprogrammer10/connorcode/blob/master/src/analytics.rs"
	>source code</a
>
on GitHub. It is completely server side and can only see basic information about your request. No tracking
cookies are ever used, and the analytics data is never shared or sold with any third parties.

<br /><br />
The following information <em>is</em> collected:
<ul>
	<li>Time of Request</li>
	<li>Client IP address</li>
	<li>HTTP Path</li>
	<li>User Agent</li>
	<li>Referer</li>
</ul>

<h2>Contact</h2>
Have any questions? Just want to say Hi?<br />
You can reach me at:
<ul>
	<li>
		Email:
		<a href="mailto:connor@connorcode.com" id="email">connor@connorcode.com</a>
	</li>
	<li>Discord: sigma76</li>
</ul>
