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

export class NaiveDate {
	public month: number;
	public day: number;
	public year: number;

	public constructor(date: string) {
		let parts = date.split('/');
		this.month = parseInt(parts[0]);
		this.day = parseInt(parts[1]);
		this.year = parseInt(parts[2]);
	}

	public human_date(): string {
		let ordinal = 'th';
		if (this.day % 10 === 1 && this.day !== 11) ordinal = 'st';
		if (this.day % 10 === 2 && this.day !== 12) ordinal = 'nd';
		if (this.day % 10 === 3 && this.day !== 13) ordinal = 'rd';

		return `${MONTHS[this.month - 1]} ${this.day}${ordinal}, ${this.year}`;
	}
}
