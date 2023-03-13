import { configDir } from '@tauri-apps/api/path'
import Database from 'tauri-plugin-sql-api'
import { dialogErrorMessage, deleteConfirm } from '@/utils'
import { isString, isObject } from '@/utils'
import type { TableName, TablePayload, WherePayload } from '@/types'

const dbFile = import.meta.env.DEV ? 'sql.dev.db' : 'sql.db'
const db = await Database.load(
  `sqlite:${await configDir()}/${import.meta.env.VITE_APP_NAME}/${dbFile}`
)

/**
 * sql 的字符串参数需要在加一个冒号
 * @param value 参数
 */
const getValue = (value: any) => (isString(value) ? `'${value}'` : value)

/**
 * 执行 sql 语句
 * @param sql sql 语句
 */
export const executeSQL = async (sql: string) => {
  const sliceSQL = sql.slice(0, 6)

  try {
    if (sliceSQL === 'SELECT') {
      return await db.select(sql)
    } else {
      await db.execute(sql)
    }
  } catch (error) {
    let action

    switch (sliceSQL) {
      case 'SELECT':
        action = '获取'
        break

      case 'INSERT':
        action = '添加'
        break

      case 'UPDATE':
        action = '更新'
        break

      case 'DELETE':
        action = '删除'
        break

      default:
        action = '创建'
        break
    }

    dialogErrorMessage(`${action}数据时遇到了问题，请重试~`)
  }
}

/**
 * 初始化 sql 配置
 */
export const initSQL = async () => {
  await executeSQL(
    `
    CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, data TEXT, role_id INTEGER, time TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
    CREATE TABLE IF NOT EXISTS role (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, description TEXT, is_deleted INTEGER DEFAULT false);
    CREATE TABLE IF NOT EXISTS credit (id INTEGER PRIMARY KEY AUTOINCREMENT, history_id INTEGER, token_cost INTEGER, api_key TEXT);
    `
  )

  await insertSQL('role', {
    name: import.meta.env.VITE_DEFAULT_ROLE_NAME,
    description: import.meta.env.VITE_DEFAULT_ROLE_DESCRIPTION
  })

  for (const item of [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) {
    await insertSQL('role', {
      name: import.meta.env.VITE_DEFAULT_ROLE_NAME + item,
      description: import.meta.env.VITE_DEFAULT_ROLE_DESCRIPTION
    })
  }
}

/**
 * 查找的 sql 语句
 * @param tableName 表名称
 * @returns
 */
export const selectSQL = async (
  tableName: TableName,
  wherePayload?: WherePayload[]
) => {
  let whereCondition = ''

  if (wherePayload) {
    const newWherePayload = wherePayload.reduce((payload, { key, value }) => {
      return payload.concat(`${key}=${getValue(value)}`)
    }, [] as string[])

    whereCondition = `WHERE ${newWherePayload.join(' AND ')}`
  }

  return (await executeSQL(
    `SELECT * FROM ${tableName} ${whereCondition} ORDER BY id DESC;`
  )) as TablePayload[]
}

/**
 * 添加的 sql 语句
 * @param tableName 表名称
 * @param payload 添加的数据
 */
export const insertSQL = async (
  tableName: TableName,
  payload: TablePayload
) => {
  if (tableName === 'role') {
    const findPayload = Object.keys(payload).reduce((result, key) => {
      const newKey = key as keyof TablePayload

      return result.concat({ key: newKey, value: payload[newKey] })
    }, [] as WherePayload[])

    const findRole = await selectSQL('role', findPayload)

    if (findRole.length) return
  }

  const insertKeys = [],
    insertValues = []

  for (const key in payload) {
    insertKeys.push(key)

    let value = payload[key as keyof typeof payload]

    if (isObject(value)) {
      value = JSON.stringify(value)
    }

    insertValues.push(getValue(value))
  }

  await executeSQL(
    `INSERT INTO ${tableName} (${insertKeys.join()}) VALUES (${insertValues.join()});`
  )
}

/**
 * 更新的 sql 语句
 * @param tableName 表名称
 * @param payload 修改的数据
 */
export const updateSQL = async (
  tableName: TableName,
  payload: TablePayload
) => {
  const newPayload = { ...payload }

  delete newPayload.id

  const updateParams: string[] = []

  for (const key in newPayload) {
    let value = newPayload[key as keyof typeof newPayload]

    if (isObject(value)) {
      value = JSON.stringify(value)
    }

    updateParams.push(`${key}=${getValue(value)}`)
  }

  await executeSQL(
    `UPDATE ${tableName} SET ${updateParams.join()} WHERE id=${payload.id};`
  )
}

/**
 * 删除的 sql 语句
 * @param tableName 表名称
 * @param id 删除数据的 id
 */
export const deleteSQL = async (tableName: TableName, id?: number) => {
  const isDelete = await deleteConfirm()

  if (!isDelete) return

  if (id) {
    const findItem = await selectSQL(tableName, [{ key: 'id', value: id }])

    if (!findItem.length) return

    await executeSQL(`DELETE FROM ${tableName} WHERE id=${id};`)
  } else {
    await executeSQL(`DELETE FROM ${tableName};`)
  }
}