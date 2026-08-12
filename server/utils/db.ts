import { Kysely, MysqlDialect } from 'kysely'
import { createPool } from 'mysql2'
import { Database } from '../types/db'

const mysqlPool = createPool({
  host: process.env.DB_HOST || 'localhost',
  user: process.env.DB_USER || 'root',
  password: process.env.DB_PASSWORD,
  database: process.env.DB_NAME,
  connectionLimit: 1
})

export const db = new Kysely<Database>({
  dialect: new MysqlDialect({
    pool: mysqlPool
  })
})
