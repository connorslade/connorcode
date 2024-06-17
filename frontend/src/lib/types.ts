export type ArticleInfo = {
	path: string;
	title: string;
	date: string;
	description: string;
	tags: string[];
	hero: Hero;

	hidden: boolean | null;
	word_count: number;
	views: number;
};

export type ProjectInfo = {
	slug: string;
	name: string;
	date: string;
	description: string;
	hero: Hero;

	word_count: number;
	pinned: boolean;
	views: number;

	github: string | null;
	link: string | null;
};

export type Hero = {
	image: string;
	alt: string;
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
