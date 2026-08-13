export default defineEventHandler(async (event) => {
	try {
		const body = await readBody<CreatePostPayload>(event);
		const postID = body.id;

		if (postID === undefined) {
			throw new Error("ID not found");
		}

		const result = await db.transaction().execute(async (trx) => {
			await trx
				.updateTable("posts_metadata")
				.set({
					author: body.author.trim(),
					title: body.title.trim(),
					slug: body.slug.trim(),
					category: body.category.trim(),
					status: body.status,
					published_at: body.status === "published" ? new Date() : null,
				})
				.where("posts_metadata.id", "=", postID)
				.execute();

			await trx
				.updateTable("posts_content")
				.set({ body: body.body })
				.where("posts_content.post_id", "=", postID)
				.execute();

			await trx
				.deleteFrom("posts_tags")
				.where("posts_tags.post_id", "=", postID)
				.execute();

			if (body.tags && body.tags.length > 0) {
				const tagsRecord = body.tags.map((obj) => ({
					post_id: postID,
					tag: obj.trim().toLowerCase(),
				}));

				await trx.insertInto("posts_tags").values(tagsRecord).execute();
			}

			return { id: postID };
		});

		return {
			success: true,
			message: "Post created cleanly across all data tables",
			postId: result.id,
		};
	} catch (error: any) {
		throw createError({
			statusCode: 500,
			statusMessage: "Failed to patch post: " + error.message,
		});
	}
});
