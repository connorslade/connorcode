export function human_file_size(size: number): string {
	const UNITS = ['B', 'kB', 'MB', 'GB', 'TB'];

	let idx = 0;
	while (size >= 1024 && idx < UNITS.length) {
		size /= 1024;
		idx++;
	}

	return `${size.toFixed(0)} ${UNITS[idx]}`;
}

export function html_to_text(html: string): string {
	return html.replace(/<[^>]+>/g, '');
}
