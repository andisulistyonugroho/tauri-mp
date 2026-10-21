import { Generated, ColumnType } from "kysely";

// Posts
export interface CreatePostPayload {
	id?: number;
	author: string;
	title: string;
	slug: string;
	category: string;
	tags: string[];
	body: string; // Markdown text string
	status: "draft" | "published" | "archived";
}

export interface PostMetadataTable {
	id?: number;
	author: string;
	slug: string;
	title: string;
	category: string;
	tags: string;
	status: "draft" | "published" | "archived";
	published_at: Date | null;
	created_at: Generated<Date>;
	updated_at: Generated<Date>;
}

export interface PostContentTable {
	post_id: number | null;
	body: string;
}

export interface PostTagsTable {
	post_id: number;
	tag: string;
}

export interface Database {
	posts_metadata: PostMetadataTable;
	posts_content: PostContentTable;
	posts_tags: PostTagsTable;
}
