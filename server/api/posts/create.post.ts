// import { dbs } from "../../utils/db";

interface CreatePostPayload {
	author_id: number;
	title: string;
	slug: string;
	category: string;
	tags: string[];
	body: string; // Markdown text string
	status: "draft" | "published" | "archived";
}

export default defineEventHandler(async (event) => {
	try {
		const body = await readBody<CreatePostPayload>(event);
		const result = await db.transaction().execute(async (trx) => {
			const metaData = await trx
				.insertInto("posts_metadata")
				.values({
					author_id: body.author_id,
					title: body.title,
					slug: body.slug,
					category: body.category,
					status: body.status,
					published_at: body.status === "published" ? new Date() : null,
				})
				.executeTakeFirstOrThrow();
			const postId = Number(metaData.insertId);

			await trx
				.insertInto("posts_content")
				.values({ post_id: postId, body: body.body })
				.execute();

			if (body.tags && body.tags.length > 0) {
				const tagsRecord = body.tags.map((obj) => ({
					post_id: postId,
					tag: obj.trim().toLowerCase(),
				}));

				await trx.insertInto("posts_tags").values(tagsRecord).execute();
			}

			return { id: postId };
		});

		return {
			success: true,
			message: "Post created cleanly across all data tables",
			postId: result.id,
		};
	} catch (error: any) {
		throw createError({
			statusCode: 500,
			statusMessage: "Failed to publish post: " + error.message,
		});
	}
});
