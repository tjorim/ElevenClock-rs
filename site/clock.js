// A small JavaScript copy of the app's format language (crates/clock-core/FORMAT.md),
// used only to preview formats on this page.
(function () {
  "use strict";

  const DAYS = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
  const MONTHS = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
  const pad = (n) => String(n).padStart(2, "0");

  function isoWeek(date) {
    const day = date.getDay() || 7;
    const thursday = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate() + 4 - day));
    const year = thursday.getUTCFullYear();
    const week = Math.ceil(((thursday - Date.UTC(year, 0, 1)) / 86400000 + 1) / 7);
    return { year, week, day };
  }

  function render(format, date) {
    const iso = isoWeek(date);
    const hour12 = date.getHours() % 12 || 12;
    const tokens = {
      Y: String(date.getFullYear()), y: pad(date.getFullYear() % 100),
      G: String(iso.year), g: pad(iso.year % 100),
      m: pad(date.getMonth() + 1), d: pad(date.getDate()),
      H: pad(date.getHours()), I: pad(hour12),
      M: pad(date.getMinutes()), S: pad(date.getSeconds()),
      p: date.getHours() < 12 ? "AM" : "PM",
      V: pad(iso.week), u: String(iso.day),
      a: DAYS[iso.day - 1].slice(0, 3), A: DAYS[iso.day - 1],
      b: MONTHS[date.getMonth()].slice(0, 3), B: MONTHS[date.getMonth()],
      "%": "%",
    };
    return format.replace(/%(.?)/g, (match, token) => (token in tokens ? tokens[token] : match));
  }

  // Hero: cycle through the presets on a live clock.
  const heroClock = document.getElementById("hero-clock");
  const heroCaption = document.getElementById("hero-caption");
  const showcase = [
    ["%g%V.%u", (d) => {
      const iso = isoWeek(d);
      return `ISO week date: ${iso.year}, week ${iso.week}, ${DAYS[iso.day - 1]}`;
    }],
    ["%H:%M\n%g%V.%u", () => "Time on top, ISO week date below"],
    ["%a %d %b %H:%M:%S", () => "Weekday, date and seconds"],
    ["%y%V.%u", () => "Calendar year with ISO week"],
  ];
  let current = 0;
  let shownSince = Date.now();
  const still = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  function tick() {
    const now = new Date();
    if (!still && Date.now() - shownSince > 4000) {
      current = (current + 1) % showcase.length;
      shownSince = Date.now();
    }
    const [format, caption] = showcase[current];
    heroClock.textContent = render(format, now);
    heroCaption.textContent = caption(now);
    updatePreview(now);
  }

  // Playground.
  const input = document.getElementById("format");
  const preview = document.getElementById("preview-clock");
  function updatePreview(now) {
    const lines = input.value.split("\n").slice(0, 2);
    preview.textContent = render(lines.join("\n"), now || new Date());
  }
  input.addEventListener("input", () => updatePreview());
  document.querySelectorAll(".presets button").forEach((button) => {
    button.addEventListener("click", () => {
      input.value = button.dataset.format;
      updatePreview();
      input.focus();
    });
  });

  tick();
  setInterval(tick, 1000);
})();
