<h1 align="center">XClock</h1>

<div align="center">
  <img src="https://xscriptor.github.io/badges/languages/rust.svg" alt="Rust" />
  <img src="https://xscriptor.github.io/badges/languages/shell.svg" alt="Shell" />
  <img src="https://xscriptor.github.io/badges/languages/powershell.svg" alt="PowerShell" />
  <p><em>A terminal-based clock application written in Rust, inspired by <code>ttyclock</code>. It features a digital clock with ASCII art, countdown timers, and various customization options.</em></p>
  <p><img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/clockface.svg" width="30" alt="XClock SVG" /></p>
</div>

<h2 id="menu">Menu</h2>
<ul>
  <li><a href="#preview">Preview</a></li>
  <li><a href="#features">Features</a></li>
  <li><a href="#installation">Installation</a></li>
  <li><a href="#installation-linux-macos">Linux / macOS</a></li>
  <li><a href="#installation-windows">Windows</a></li>
  <li><a href="#uninstallation">Uninstallation</a></li>
  <li><a href="#usage">Usage</a></li>
  <li><a href="#options">Options</a></li>
  <li><a href="#examples">Examples</a></li>
  <li><a href="#development">Development</a></li>
</ul>

<h2 id="preview">Preview</h2>
<p align="center">
  <img src="https://raw.githubusercontent.com/xscriptor/xassets/main/xrepos/apps/xclock/preview01.png" alt="Main preview" width="450" />
</p>

<h2 id="features">Features</h2>
<ul>
  <li><strong>Clock Mode:</strong> Displays current time with seconds.</li>
  <li><strong>Countdown Mode:</strong> Set a timer using formats like <code>5m</code>, <code>1h30m</code>, <code>10s</code>.</li>
  <li><strong>Customization:</strong>
    <ul>
      <li>Center alignment.</li>
      <li>Colors (red, green, blue, yellow, cyan, magenta, white, black).</li>
      <li>12/24 hour format.</li>
      <li>Toggle seconds (<code>-s</code> to show).</li>
      <li>Hide/Show box borders.</li>
    </ul>
  </li>
  <li><strong>Cross-platform:</strong> Works on Linux, macOS, and Windows.</li>
</ul>

<h2 id="installation">Installation</h2>

<h3 id="installation-linux-macos">Linux / macOS</h3>
<p>You can install XClock using the provided script. It will auto-detect your OS (Arch/Ubuntu/Fedora/macOS), install Rust if missing, and then install XClock:</p>
<pre><code>./install.sh</code></pre>
<p>Or remotely:</p>
<pre><code>wget -O - https://raw.githubusercontent.com/xscriptor/xclock/main/install.sh | bash</code></pre>

<h3 id="installation-windows">Windows</h3>
<p>Run the PowerShell script:</p>
<pre><code>./install.ps1</code></pre>

<h3 id="uninstallation">Uninstallation</h3>
<ul>
  <li>Linux/macOS: <code>./uninstall.sh</code></li>
  <li>Windows: <code>./uninstall.ps1</code></li>
</ul>

<h2 id="usage">Usage</h2>
<pre><code>xclock [OPTIONS]</code></pre>

<h3 id="options">Options</h3>
<table>
  <thead>
    <tr>
      <th>Option</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>-c</code>, <code>--center</code></td>
      <td>Center the clock on the screen</td>
    </tr>
    <tr>
      <td><code>-C</code>, <code>--countdown &lt;TIME&gt;</code></td>
      <td>Enable countdown mode (e.g., <code>5m</code>, <code>1h30m</code>)</td>
    </tr>
    <tr>
      <td><code>-s</code>, <code>--seconds</code></td>
      <td>Show seconds (default: off)</td>
    </tr>
    <tr>
      <td><code>-r</code>, <code>--color &lt;COLOR&gt;</code></td>
      <td>Set color (default: green)</td>
    </tr>
    <tr>
      <td><code>-t</code>, <code>--twelve-hour</code></td>
      <td>Use 12-hour format</td>
    </tr>
    <tr>
      <td><code>-B</code>, <code>--no-box</code></td>
      <td>Hide the box borders</td>
    </tr>
    <tr>
      <td><code>-h</code>, <code>--help</code></td>
      <td>Print help</td>
    </tr>
  </tbody>
</table>

<p>Seconds are disabled by default to mimic <code>ttyclock</code>. Use <code>-s</code> to show them.</p>

<h3 id="examples">Examples</h3>
<p>Run a centered green clock:</p>
<pre><code>xclock -c -r green</code></pre>

<p>Run a 5-minute countdown:</p>
<pre><code>xclock -C 5m</code></pre>

<p>Run a clock without box and seconds:</p>
<pre><code>xclock -B</code></pre>

<h2 id="development">Development</h2>
<p>Built with Rust, using <code>ratatui</code> and <code>crossterm</code>.</p>
<pre><code>cargo run --release</code></pre>

<div align="center">
<h2>X</h2>

<a href="https://dev.xscriptor.com"><img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/ellipsis.svg" width="24" alt="X Web" /></a> & <a href="https://github.com/xscriptor"><img src="https://xscriptor.github.io/icons/icons/code/product-design/xsvg/github.svg" width="24" alt="X Profile" /></a>
</div>