use axum::response::Html;

pub async fn dashboard_page() -> Html<String> {
    Html(DASHBOARD_HTML.to_string())
}

const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dashboard - Redirector Admin</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script>
        tailwind.config = {
            darkMode: 'class',
            theme: {
                extend: {
                    colors: {
                        brown: {
                            900: '#1a1412',
                            800: '#2d2320',
                            700: '#3d322d',
                            600: '#4d413a',
                        }
                    }
                }
            }
        }
    </script>
    <style>
        .theme-brown .bg-slate-900 { background-color: #1a1412 !important; }
        .theme-brown .bg-slate-800 { background-color: #2d2320 !important; }
        .theme-brown .bg-slate-700 { background-color: #3d322d !important; }
    </style>
    <script>
        // Theme handling
        function getTheme() {
            return localStorage.getItem('theme') || 'light';
        }
        function applyTheme(theme) {
            document.documentElement.classList.remove('dark', 'theme-brown');
            if (theme === 'dark') {
                document.documentElement.classList.add('dark');
            } else if (theme === 'brown') {
                document.documentElement.classList.add('dark', 'theme-brown');
            }
            localStorage.setItem('theme', theme);
            updateChartColors();
        }
        applyTheme(getTheme());
    </script>
</head>
<body class="min-h-screen bg-slate-50 dark:bg-slate-900">
    <!-- Header -->
    <header class="bg-white dark:bg-slate-800 shadow-sm border-b border-slate-200 dark:border-slate-700">
        <div class="max-w-7xl mx-auto px-4 py-4 flex items-center justify-between">
            <div class="flex items-center gap-3">
                <span class="text-2xl">🔄</span>
                <h1 class="text-xl font-bold text-slate-900 dark:text-white">redirector</h1>
                <span class="text-sm text-slate-500 dark:text-slate-400">admin</span>
            </div>
            <div class="flex items-center gap-4">
                <!-- Theme selector -->
                <div class="flex gap-1 bg-slate-100 dark:bg-slate-700 rounded-lg p-1">
                    <button onclick="applyTheme('light')" class="p-1.5 rounded hover:bg-white dark:hover:bg-slate-600 transition-colors" title="Light">☀️</button>
                    <button onclick="applyTheme('dark')" class="p-1.5 rounded hover:bg-white dark:hover:bg-slate-600 transition-colors" title="Dark">🌙</button>
                    <button onclick="applyTheme('brown')" class="p-1.5 rounded hover:bg-white dark:hover:bg-slate-600 transition-colors" title="Brown">🪵</button>
                </div>
                <!-- Logout -->
                <form action="/admin/logout" method="POST">
                    <button type="submit" class="text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white transition-colors">
                        Logout
                    </button>
                </form>
            </div>
        </div>
    </header>

    <main class="max-w-7xl mx-auto px-4 py-8">
        <!-- Stats Grid -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">Uptime</div>
                <div class="text-2xl font-bold text-slate-900 dark:text-white" id="stat-uptime">-</div>
            </div>
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">CPU</div>
                <div class="text-2xl font-bold text-slate-900 dark:text-white"><span id="stat-cpu">-</span>%</div>
            </div>
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">Memory</div>
                <div class="text-2xl font-bold text-slate-900 dark:text-white"><span id="stat-memory">-</span> MB</div>
            </div>
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <div class="text-sm text-slate-500 dark:text-slate-400 mb-1">RPS</div>
                <div class="text-2xl font-bold text-blue-600" id="stat-rps">-</div>
            </div>
        </div>

        <!-- Charts Row -->
        <div class="grid md:grid-cols-2 gap-4 mb-8">
            <!-- RPS Chart -->
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <h3 class="text-sm font-medium text-slate-500 dark:text-slate-400 mb-4">Requests per Second</h3>
                <div class="h-64">
                    <canvas id="rpsChart"></canvas>
                </div>
            </div>
            <!-- Latency Chart -->
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <h3 class="text-sm font-medium text-slate-500 dark:text-slate-400 mb-4">Latency (ms)</h3>
                <div class="h-64">
                    <canvas id="latencyChart"></canvas>
                </div>
            </div>
        </div>

        <!-- Bottom Row -->
        <div class="grid md:grid-cols-2 gap-4">
            <!-- Cache Hit Rate -->
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <h3 class="text-sm font-medium text-slate-500 dark:text-slate-400 mb-4">Cache Hit Rate</h3>
                <div class="flex items-center gap-4">
                    <div class="flex-1 bg-slate-200 dark:bg-slate-700 rounded-full h-4 overflow-hidden">
                        <div id="cache-bar" class="bg-blue-500 h-full transition-all duration-500" style="width: 0%"></div>
                    </div>
                    <span class="text-lg font-bold text-slate-900 dark:text-white" id="stat-cache">0%</span>
                </div>
                <div class="mt-4 text-sm text-slate-500 dark:text-slate-400">
                    Total requests: <span id="stat-total" class="font-medium text-slate-700 dark:text-slate-300">0</span>
                </div>
            </div>

            <!-- Recent Redirects -->
            <div class="bg-white dark:bg-slate-800 rounded-xl shadow-sm p-6">
                <h3 class="text-sm font-medium text-slate-500 dark:text-slate-400 mb-4">Recent Redirects</h3>
                <div class="space-y-2 max-h-48 overflow-y-auto" id="recent-list">
                    <div class="text-slate-400 dark:text-slate-500 text-sm">No recent redirects</div>
                </div>
            </div>
        </div>
    </main>

    <script>
        // Chart setup
        const maxDataPoints = 60;
        const rpsData = { labels: [], data: [] };
        const latencyData = { labels: [], p50: [], p95: [], p99: [] };

        function getChartColors() {
            const isDark = document.documentElement.classList.contains('dark');
            return {
                text: isDark ? '#94a3b8' : '#64748b',
                grid: isDark ? '#334155' : '#e2e8f0',
                blue: '#3b82f6',
                green: '#22c55e',
                yellow: '#eab308',
                red: '#ef4444',
            };
        }

        const colors = getChartColors();

        const rpsChart = new Chart(document.getElementById('rpsChart'), {
            type: 'line',
            data: {
                labels: rpsData.labels,
                datasets: [{
                    label: 'RPS',
                    data: rpsData.data,
                    borderColor: colors.blue,
                    backgroundColor: colors.blue + '20',
                    fill: true,
                    tension: 0.3,
                    pointRadius: 0,
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false } },
                scales: {
                    x: { display: false },
                    y: {
                        beginAtZero: true,
                        grid: { color: colors.grid },
                        ticks: { color: colors.text }
                    }
                }
            }
        });

        const latencyChart = new Chart(document.getElementById('latencyChart'), {
            type: 'line',
            data: {
                labels: latencyData.labels,
                datasets: [
                    { label: 'p50', data: latencyData.p50, borderColor: colors.green, tension: 0.3, pointRadius: 0 },
                    { label: 'p95', data: latencyData.p95, borderColor: colors.yellow, tension: 0.3, pointRadius: 0 },
                    { label: 'p99', data: latencyData.p99, borderColor: colors.red, tension: 0.3, pointRadius: 0 },
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'top',
                        labels: { color: colors.text, boxWidth: 12 }
                    }
                },
                scales: {
                    x: { display: false },
                    y: {
                        beginAtZero: true,
                        grid: { color: colors.grid },
                        ticks: { color: colors.text }
                    }
                }
            }
        });

        function updateChartColors() {
            const colors = getChartColors();
            rpsChart.options.scales.y.grid.color = colors.grid;
            rpsChart.options.scales.y.ticks.color = colors.text;
            latencyChart.options.scales.y.grid.color = colors.grid;
            latencyChart.options.scales.y.ticks.color = colors.text;
            latencyChart.options.plugins.legend.labels.color = colors.text;
            rpsChart.update('none');
            latencyChart.update('none');
        }

        // Format uptime
        function formatUptime(secs) {
            const days = Math.floor(secs / 86400);
            const hours = Math.floor((secs % 86400) / 3600);
            const mins = Math.floor((secs % 3600) / 60);
            if (days > 0) return `${days}d ${hours}h`;
            if (hours > 0) return `${hours}h ${mins}m`;
            return `${mins}m`;
        }

        // Format URL for display
        function formatUrl(url, maxLen = 40) {
            if (url.length <= maxLen) return url;
            return url.substring(0, maxLen) + '...';
        }

        // SSE connection
        const evtSource = new EventSource('/admin/events');

        evtSource.onmessage = function(event) {
            const data = JSON.parse(event.data);
            const now = new Date().toLocaleTimeString();

            // Update stats
            document.getElementById('stat-uptime').textContent = formatUptime(data.system.uptime_secs);
            document.getElementById('stat-cpu').textContent = data.system.cpu_percent.toFixed(1);
            document.getElementById('stat-memory').textContent = data.system.memory_mb;
            document.getElementById('stat-rps').textContent = data.app.rps.toFixed(0);
            document.getElementById('stat-cache').textContent = (data.app.cache_hit_rate * 100).toFixed(1) + '%';
            document.getElementById('cache-bar').style.width = (data.app.cache_hit_rate * 100) + '%';
            document.getElementById('stat-total').textContent = data.app.total_requests.toLocaleString();

            // Update RPS chart
            rpsData.labels.push(now);
            rpsData.data.push(data.app.rps);
            if (rpsData.labels.length > maxDataPoints) {
                rpsData.labels.shift();
                rpsData.data.shift();
            }
            rpsChart.update('none');

            // Update latency chart
            latencyData.labels.push(now);
            latencyData.p50.push(data.app.latency_p50_ms);
            latencyData.p95.push(data.app.latency_p95_ms);
            latencyData.p99.push(data.app.latency_p99_ms);
            if (latencyData.labels.length > maxDataPoints) {
                latencyData.labels.shift();
                latencyData.p50.shift();
                latencyData.p95.shift();
                latencyData.p99.shift();
            }
            latencyChart.update('none');

            // Update recent redirects
            if (data.recent && data.recent.length > 0) {
                const list = document.getElementById('recent-list');
                list.innerHTML = data.recent.map(r => `
                    <div class="flex items-center gap-2 text-sm py-1 border-b border-slate-100 dark:border-slate-700 last:border-0">
                        <code class="text-blue-600 dark:text-blue-400 font-mono">${r.hashid}</code>
                        <span class="text-slate-400">→</span>
                        <span class="text-slate-600 dark:text-slate-300 truncate" title="${r.url}">${formatUrl(r.url)}</span>
                    </div>
                `).join('');
            }
        };

        evtSource.onerror = function() {
            console.error('SSE connection lost, reconnecting...');
        };
    </script>
</body>
</html>"##;
