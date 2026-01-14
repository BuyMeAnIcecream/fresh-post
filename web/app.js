async function fetchJson(url, options = {}) {
  const res = await fetch(url, {
    headers: { "Content-Type": "application/json" },
    ...options,
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || res.statusText);
  }
  return res.json();
}

function setStatus(el, message, isError = false) {
  if (!el) return;
  el.textContent = message;
  el.classList.toggle("error", isError);
}

async function loadConfig() {
  const config = await fetchJson("/api/config");
  const keywords = document.getElementById("keywords");
  const location = document.getElementById("location");
  const remote = document.getElementById("remote");
  const salaryMin = document.getElementById("salary_min");
  const intervalHours = document.getElementById("interval_hours");

  if (keywords) keywords.value = config.search.keywords || "";
  if (location) location.value = config.search.location || "";
  if (remote) remote.checked = !!config.search.remote;
  if (salaryMin) salaryMin.value = config.search.salary_min || 0;
  if (intervalHours) intervalHours.value = (config.schedule && config.schedule.interval_hours) || 4;
}

async function saveConfig(event) {
  event.preventDefault();
  const statusEl = document.getElementById("config-status");

  const payload = {
    search: {
      keywords: document.getElementById("keywords").value.trim(),
      location: document.getElementById("location").value.trim(),
      remote: document.getElementById("remote").checked,
      salary_min: Number.parseInt(document.getElementById("salary_min").value, 10) || 0,
    },
    schedule: {
      interval_hours: Number.parseInt(document.getElementById("interval_hours").value, 10) || 4,
    },
  };

  try {
    await fetchJson("/api/config", {
      method: "POST",
      body: JSON.stringify(payload),
    });
    setStatus(statusEl, "Config saved.");
  } catch (err) {
    setStatus(statusEl, `Failed to save config: ${err.message}`, true);
  }
}

async function runNow() {
  const statusEl = document.getElementById("config-status");
  setStatus(statusEl, "Running scrape...");
  try {
    const summary = await fetchJson("/api/run", { method: "POST" });
    setStatus(
      statusEl,
      `Done. New jobs: ${summary.new_jobs}, Today: ${summary.today_jobs}.`
    );
  } catch (err) {
    setStatus(statusEl, `Run failed: ${err.message}`, true);
  }
}

async function loadJobs() {
  const listEl = document.getElementById("jobs-list");
  const metaEl = document.getElementById("jobs-meta");
  if (!listEl || !metaEl) return;

  try {
    const snapshot = await fetchJson("/api/jobs");
    const updated = snapshot.updated_at
      ? new Date(snapshot.updated_at).toLocaleString()
      : "No runs yet";
    metaEl.textContent = `Updated: ${updated} • Total today: ${snapshot.jobs.length} • New: ${snapshot.new_jobs.length}`;

    listEl.innerHTML = "";
    if (snapshot.jobs.length === 0) {
      const li = document.createElement("li");
      li.textContent = "No jobs found yet.";
      listEl.appendChild(li);
      return;
    }

    snapshot.jobs.forEach((job) => {
      const li = document.createElement("li");
      li.className = "job-item";

      const title = document.createElement("div");
      title.className = "job-title";
      title.textContent = job.title;

      const meta = document.createElement("div");
      meta.className = "job-meta";
      meta.textContent = `${job.company} • ${job.location}`;

      const link = document.createElement("a");
      link.href = job.url;
      link.target = "_blank";
      link.rel = "noopener";
      link.textContent = "View on LinkedIn";

      li.appendChild(title);
      li.appendChild(meta);
      li.appendChild(link);
      listEl.appendChild(li);
    });
  } catch (err) {
    metaEl.textContent = `Failed to load jobs: ${err.message}`;
  }
}

function init() {
  const form = document.getElementById("config-form");
  if (form) {
    form.addEventListener("submit", saveConfig);
    loadConfig();

    const runButton = document.getElementById("run-now");
    if (runButton) {
      runButton.addEventListener("click", runNow);
    }
  }

  loadJobs();
}

document.addEventListener("DOMContentLoaded", init);
