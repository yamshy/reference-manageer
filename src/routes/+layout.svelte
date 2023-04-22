<script>
	import { page } from '$app/stores';
	import '../app.postcss';

	const inactiveClass = 'text-base-content hover:bg-base-200 hover:text-base-100';
	const activeClass =
		'text-primary-content bg-primary hover:bg-primary-focus hover:text-primary-content';
	const url = $page.url.pathname;
</script>

<html data-theme="corporate" lang="en">
	<div class="drawer drawer-mobile">
		<input id="main-drawer" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col items-center justify-center">
			<slot />
			<label for="main-drawer" class="btn btn-ghost drawer-button lg:hidden absolute top-0 left-0"
				><svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
					/>
				</svg>
			</label>
		</div>
		<div class="drawer-side">
			<label for="main-drawer" class="drawer-overlay" />
			<ul class="menu menu-compact p-1 bg-base-200 text-base-content">
				<li>
					<a class={url === '/documents' ? activeClass : inactiveClass} href="/documents"
						>Documents</a
					>
				</li>
			</ul>
		</div>
	</div>
</html>
