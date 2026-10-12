use axum::{
    response::Html,
    routing::get,
    Router,
};

const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="MEQKBANK financial services information">
  <meta name="theme-color" content="#e9f7ef">
  <title>MEQKBANK — Financial Services</title>

  <style>
    :root {
      color-scheme: light;
      --ink: #18332b;
      --muted: #526860;
      --green: #236b4d;
      --green-soft: #e9f7ef;
      --blue-soft: #e8f5fc;
      --paper: #fbfdfb;
      --line: #dce9e1;
      --warning: #fff5df;
      --warning-ink: #674a12;
      --radius: 18px;
      --shadow: 0 16px 40px rgba(24, 51, 43, .08);
    }

    * { box-sizing: border-box; }

    html { scroll-behavior: smooth; }

    body {
      margin: 0;
      background:
        radial-gradient(ellipse at 85% 5%, #d9effb 0, transparent 34rem),
        radial-gradient(ellipse at 5% 25%, #e3f5e9 0, transparent 32rem),
        var(--paper);
      color: var(--ink);
      font: 16px/1.65 system-ui, -apple-system, "Segoe UI", sans-serif;
    }

    a { color: var(--green); }
    a:focus-visible {
      outline: 3px solid #2585a8;
      outline-offset: 4px;
      border-radius: 4px;
    }

    .wrap { width: min(1100px, calc(100% - 40px)); margin-inline: auto; }

    .site-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 24px;
      padding-block: 22px;
    }

    .brand {
      display: inline-flex;
      align-items: center;
      gap: 11px;
      color: var(--ink);
      text-decoration: none;
      font-weight: 800;
      letter-spacing: .08em;
    }

    .brand-mark {
      display: grid;
      width: 38px;
      aspect-ratio: 1;
      place-items: center;
      border-radius: 12px;
      background: var(--green);
      color: white;
      font-size: 20px;
    }

    nav { display: flex; flex-wrap: wrap; gap: 22px; }
    nav a { color: var(--ink); text-decoration: none; font-size: .94rem; }
    nav a:hover { color: var(--green); text-decoration: underline; }

    .hero {
      position: relative;
      overflow: hidden;
      margin-top: 24px;
      padding: clamp(34px, 7vw, 76px);
      border: 1px solid rgba(220, 233, 225, .9);
      border-radius: 28px;
      background: linear-gradient(120deg, rgba(255,255,255,.96), rgba(239,249,250,.9));
      box-shadow: var(--shadow);
    }

    .hero::after {
      content: "∿";
      position: absolute;
      right: 5%;
      bottom: -105px;
      color: rgba(35, 107, 77, .08);
      font: 300 350px/1 Georgia, serif;
      transform: rotate(-18deg);
      pointer-events: none;
    }

    .eyebrow {
      margin: 0 0 12px;
      color: var(--green);
      font-size: .78rem;
      font-weight: 800;
      letter-spacing: .15em;
      text-transform: uppercase;
    }

    h1, h2, h3 { line-height: 1.2; }
    h1 {
      max-width: 720px;
      margin: 0;
      font-family: Georgia, "Times New Roman", serif;
      font-size: clamp(2.5rem, 7vw, 5rem);
      font-weight: 500;
      letter-spacing: -.045em;
    }

    .hero-copy {
      max-width: 620px;
      margin: 22px 0 28px;
      color: var(--muted);
      font-size: 1.1rem;
    }

    .button {
      display: inline-block;
      padding: 12px 18px;
      border-radius: 10px;
      background: var(--green);
      color: white;
      font-weight: 700;
      text-decoration: none;
    }

    .button:hover { background: #18563d; }

    .notice {
      display: grid;
      grid-template-columns: auto 1fr;
      gap: 14px;
      margin-top: 22px;
      padding: 18px 20px;
      border: 1px solid #f0d99d;
      border-radius: 14px;
      background: var(--warning);
      color: var(--warning-ink);
    }

    .notice strong { display: block; }
    .notice p { margin: 3px 0 0; }
    .notice-icon { font-size: 1.4rem; }

    section.content { padding-block: 58px 10px; }
    .section-heading { max-width: 660px; margin-bottom: 24px; }
    .section-heading h2 {
      margin: 0 0 8px;
      font-family: Georgia, "Times New Roman", serif;
      font-size: clamp(1.8rem, 4vw, 2.6rem);
      font-weight: 500;
    }
    .section-heading p { margin: 0; color: var(--muted); }

    .grid {
      display: grid;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 16px;
    }

    .card {
      min-height: 190px;
      padding: 24px;
      border: 1px solid var(--line);
      border-radius: var(--radius);
      background: rgba(255,255,255,.82);
    }

    .card-number {
      color: #6e9c83;
      font: 1rem Georgia, serif;
    }

    .card h3 { margin: 18px 0 8px; font-size: 1.2rem; }
    .card p { margin: 0; color: var(--muted); }

    .principle {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 28px;
      align-items: center;
      margin-top: 58px;
      padding: clamp(24px, 5vw, 44px);
      border-radius: 22px;
      background: linear-gradient(135deg, var(--green-soft), var(--blue-soft));
    }

    .principle h2 {
      margin: 0;
      font: 500 clamp(1.8rem, 4vw, 2.6rem)/1.2 Georgia, serif;
    }

    .principle p { margin: 0; color: var(--muted); }

    .terms {
      padding: 24px;
      border: 1px solid var(--line);
      border-radius: var(--radius);
      background: white;
    }

    .terms h3 { margin: 0 0 8px; }
    .terms p { margin: 0 0 14px; color: var(--muted); }
    .terms p:last-child { margin-bottom: 0; }

    .contact {
      display: flex;
      flex-wrap: wrap;
      justify-content: space-between;
      gap: 22px;
      padding: 28px 0 48px;
    }

    .contact p { margin: 5px 0; }
    footer {
      padding: 20px 0;
      border-top: 1px solid var(--line);
      color: var(--muted);
      font-size: .9rem;
    }

    @media (max-width: 720px) {
      .site-header { align-items: flex-start; flex-direction: column; }
      .grid { grid-template-columns: 1fr; }
      .principle { grid-template-columns: 1fr; }
    }

    @media (prefers-reduced-motion: reduce) {
      html { scroll-behavior: auto; }
    }
  </style>
</head>

<body>
  <header class="site-header wrap">
    <a class="brand" href="#" aria-label="MEQKBANK home">
      <span class="brand-mark" aria-hidden="true">M</span>
      <span>MEQKBANK</span>
    </a>
    <nav aria-label="Main navigation">
      <a href="#services">Services</a>
      <a href="#principles">Our approach</a>
      <a href="#terms">Terms &amp; policy</a>
      <a href="#contact">Contact</a>
    </nav>
  </header>

  <main class="wrap">
    <section class="hero" aria-labelledby="hero-title">
      <p class="eyebrow">Financial services information</p>
      <h1 id="hero-title">A clearer way to think about finance.</h1>
      <p class="hero-copy">
        MEQKBANK presents information about savings, investment, shares and
        securities through a calm, considered experience.
      </p>
      <a class="button" href="#services">Explore services</a>
    </section>

    <aside class="notice" role="note" aria-label="Important payment warning">
      <span class="notice-icon" aria-hidden="true">⚠</span>
      <div>
        <strong>Do not send money through this website.</strong>
        <p>
          This site does not accept deposits, payments, transfers, or account
          credentials. Do not share bank details, passwords, or financial
          documents here.
        </p>
      </div>
    </aside>

    <section class="content" id="services">
      <div class="section-heading">
        <p class="eyebrow">What we cover</p>
        <h2>Services, clearly organized.</h2>
        <p>Explore these topics as general information, not as a promise of a financial product.</p>
      </div>

      <div class="grid">
        <article class="card">
          <span class="card-number">01 / PLAN</span>
          <h3>Savings</h3>
          <p>Ideas and information for setting goals and building a savings habit.</p>
        </article>
        <article class="card">
          <span class="card-number">02 / GROW</span>
          <h3>Investment</h3>
          <p>General investment concepts, including risk, time horizons and diversification.</p>
        </article>
        <article class="card">
          <span class="card-number">03 / UNDERSTAND</span>
          <h3>Shares &amp; securities</h3>
          <p>An introduction to shares and other securities, and the risks they may involve.</p>
        </article>
      </div>
    </section>

    <section class="principle" id="principles">
      <h2>Good financial decisions begin with understanding.</h2>
      <p>
        We believe useful financial communication should be accessible,
        transparent and careful about uncertainty. Take time to verify
        providers and seek qualified advice suited to your circumstances.
      </p>
    </section>

    <section class="content" id="terms">
      <div class="section-heading">
        <p class="eyebrow">Read before relying</p>
        <h2>Brief terms &amp; policy</h2>
      </div>
      <div class="terms">
        <h3>Information, not financial advice</h3>
        <p>
          Content on this website is general information only. It is not
          financial, investment, tax or legal advice, and it does not take
          account of your objectives or circumstances.
        </p>
        <h3>Risk and verification</h3>
        <p>
          Investments can fall as well as rise, and you may lose some or all
          of the money invested. Returns are not guaranteed. Independently
          verify any financial provider, its authorization and its terms
          before taking action.
        </p>
        <h3>Privacy and security</h3>
        <p>
          This informational page has no financial transaction or account
          submission feature. Do not send money or submit sensitive financial
          information through it. Contact the site operator for questions
          about how any information you email may be handled.
        </p>
        <h3>Important limitation</h3>
        <p>
          This brief notice is not a complete contract, privacy policy or
          regulatory disclosure. Obtain jurisdiction-specific legal and
          compliance review before using this site to promote or provide
          regulated financial services.
        </p>
      </div>
    </section>

    <section class="content" id="contact">
      <div class="section-heading">
        <p class="eyebrow">Get in touch</p>
        <h2>Contact &amp; official social link</h2>
      </div>
      <div class="contact">
        <div>
          <strong>Overseeing director</strong>
          <p><a href="mailto:hanok0fficial@proton.me">hanok0fficial@proton.me</a></p>
        </div>
        <div>
          <strong>Social platform</strong>
          <p>
            <a href="https://www.facebook.com/profile.php?id=61591134360882"
               target="_blank" rel="noopener noreferrer">MEQKBANK on Facebook</a>
          </p>
        </div>
      </div>
    </section>
  </main>

  <footer>
    <div class="wrap">
      © 2026 MEQKBANK · Informational website · No payments accepted
    </div>
  </footer>
</body>
</html>
"#;

async fn home() -> Html<&'static str> {
    Html(PAGE)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));
    let address = "0.0.0.0:3000";

    println!("MEQKBANK is available at http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Could not bind to port 3000");

    axum::serve(listener, app)
        .await
        .expect("Web server stopped unexpectedly");
}