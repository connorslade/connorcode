export type ArticleInfo = {
	date: string;
	description: string;
	path: string;
	tags: string[];
	title: string;
	word_count: number;
};

export class Heading {
	level: number;
	text: string;
	id: string;

	public constructor(level: number, text: string, id: string) {
		this.level = level;
		this.text = text;
		this.id = id;
	}
}
