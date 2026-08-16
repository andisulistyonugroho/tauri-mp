import { sql } from "kysely";
export default defineEventHandler(async (event) => {
	try {
		const paramId = getRouterParam(event, "id");

		// 2. Parse and validate the ID structure
		const postID = paramId ? parseInt(paramId, 10) : undefined;

		// 3. Type guard assertion: fail fast if ID is invalid or missing
		if (!postID || isNaN(postID)) {
			throw createError({
				statusCode: 400,
				statusMessage: "Invalid or missing Post ID parameter.",
			});
		}

		const post = await db
			.selectFrom("posts_content as pc")
			.select(["pc.post_id", "pc.body"])
			.where("pc.post_id", "=", postID)
			.executeTakeFirstOrThrow();

		return post;
	} catch (error: any) {
		throw createError({
			statusCode: 500,
			statusMessage: error.message,
		});
	}
});
