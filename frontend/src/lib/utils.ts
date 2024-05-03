export function human_file_size(size: number): string {
	const UNITS = ['B', 'kB', 'MB', 'GB', 'TB'];

	let idx = 0;
	while (size >= 1024 && idx < UNITS.length) {
		size /= 1024;
		idx++;
	}

	return `${size.toFixed(0)} ${UNITS[idx]}`;
}

// date: mm/dd/yyyy
export function human_date(date: string): string {
	const MONTHS: string[] = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];

	const [month, day, year] = date.split('/');

	const day_int = parseInt(day);
	let ordinal = 'th';
	if (day_int % 10 === 1 && day_int !== 11) ordinal = 'st';
	if (day_int % 10 === 2 && day_int !== 12) ordinal = 'nd';
	if (day_int % 10 === 3 && day_int !== 13) ordinal = 'rd';

	return `${MONTHS[parseInt(month) - 1]} ${parseInt(day)}${ordinal}, ${year}`;
}
