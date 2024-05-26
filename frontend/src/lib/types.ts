export type ArticleInfo = {
	date: string;
	description: string;
	path: string;
	tags: string[];
	title: string;
	hidden: boolean | null;
	word_count: number;
	views: number;
};

export type ProjectInfo = {
	slug: string;
	name: string;
	date: string;
	description: string;
	word_count: number;
	pinned: boolean;
	views: number;

	github: string | null;
	link: string | null;
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
