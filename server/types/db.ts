import { Generated, ColumnType } from 'kysely'

export interface AuthorTable {
  id: Generated<number>
  display_name: string
  created_at: Generated<Date>
}

export interface PostMetadataTable {
  id: Generated<number>
  author_id: number
  slug: string
  title: string
  category: string
  status: 'draft' | 'published' | 'archived'
  published_at: Date | null
  created_at: Generated<Date>
  updated_at: Generated<Date>
}

export interface PostContentTable {
  post_id: number
  body: string
}

export interface PostTagsTable {
  post_id: number
  tag: string
}

export interface Database {
  authors: AuthorTable
  posts_metadata: PostMetadataTable
  posts_content: PostContentTable
  posts_tags: PostTagsTable
}
