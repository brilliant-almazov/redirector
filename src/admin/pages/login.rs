use axum::response::Html;

pub async fn login_page() -> Html<String> {
    Html(LOGIN_HTML.to_string())
}

const LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login - Redirector Admin</title>
    <script src="https://cdn.tailwindcss.com"></script>
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
        .theme-brown { --bg-base: #1a1412; --bg-card: #2d2320; --text-primary: #e8e0db; }
    </style>
    <script>
        // Theme handling
        const themes = ['light', 'dark', 'brown'];
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
        }
        // Apply on load
        applyTheme(getTheme());
    </script>
</head>
<body class="min-h-screen bg-slate-50 dark:bg-slate-900 theme-brown:bg-brown-900 flex items-center justify-center">
    <div class="w-full max-w-md p-8">
        <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-lg p-8">
            <div class="text-center mb-8">
                <div class="text-4xl mb-2">🔄</div>
                <h1 class="text-2xl font-bold text-slate-900 dark:text-white">redirector</h1>
                <p class="text-slate-500 dark:text-slate-400 mt-1">Admin Dashboard</p>
            </div>

            <form action="/admin/login" method="POST" class="space-y-6">
                <div>
                    <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        Username
                    </label>
                    <input
                        type="text"
                        name="username"
                        required
                        class="w-full px-4 py-3 rounded-lg border border-slate-300 dark:border-slate-600
                               bg-white dark:bg-slate-700 text-slate-900 dark:text-white
                               focus:ring-2 focus:ring-blue-500 focus:border-transparent
                               transition-colors"
                        placeholder="Enter username"
                    >
                </div>

                <div>
                    <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                        Password
                    </label>
                    <input
                        type="password"
                        name="password"
                        required
                        class="w-full px-4 py-3 rounded-lg border border-slate-300 dark:border-slate-600
                               bg-white dark:bg-slate-700 text-slate-900 dark:text-white
                               focus:ring-2 focus:ring-blue-500 focus:border-transparent
                               transition-colors"
                        placeholder="Enter password"
                    >
                </div>

                <button
                    type="submit"
                    class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 text-white font-medium
                           rounded-lg transition-colors focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                >
                    Sign In
                </button>
            </form>

            <!-- Theme selector -->
            <div class="mt-6 pt-6 border-t border-slate-200 dark:border-slate-700">
                <div class="flex justify-center gap-2">
                    <button onclick="applyTheme('light')"
                            class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
                            title="Light theme">
                        ☀️
                    </button>
                    <button onclick="applyTheme('dark')"
                            class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
                            title="Dark theme">
                        🌙
                    </button>
                    <button onclick="applyTheme('brown')"
                            class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
                            title="Brown theme">
                        🪵
                    </button>
                </div>
            </div>
        </div>
    </div>
</body>
</html>"#;
