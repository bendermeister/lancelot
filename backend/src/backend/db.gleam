import backend/context
import backend/log
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import pog

pub fn query_error_format(error: pog.QueryError) -> String {
  case error {
    pog.ConnectionUnavailable -> "connection unavailable"
    pog.ConstraintViolated(message:, constraint:, detail:) ->
      "constraint [" <> constraint <> "] " <> message <> ": " <> detail
    pog.PostgresqlError(code:, name:, message:) ->
      "[" <> code <> "] " <> name <> ": " <> message
    pog.QueryTimeout -> "query timeout"
    pog.UnexpectedArgumentCount(expected:, got:) ->
      "unexpected argument count: got: "
      <> int.to_string(got)
      <> ", expected: "
      <> int.to_string(expected)
    pog.UnexpectedArgumentType(expected:, got:) ->
      "unexpected argument type got: " <> got <> ", expected: " <> expected
    pog.UnexpectedResultType(errors) -> {
      errors
      |> list.map(fn(error) {
        "{expected: " <> error.expected <> ", got: " <> error.found <> "}"
      })
      |> string.join(", ")
      |> string.append("decode error: ", _)
    }
  }
  |> string.append("DB ERROR: ", _)
}

pub fn transaction_error_format(error: pog.TransactionError(a)) -> String {
  case error {
    pog.TransactionQueryError(error) ->
      error
      |> query_error_format
      |> string.append("TRANSACTION ", _)
    pog.TransactionRolledBack(_) -> "TRANSACTION: rolled back"
  }
}

pub fn execute(query: pog.Query(a), ctx: context.Context) -> Result(Nil, Nil) {
  fetch(query, ctx)
  |> result.replace(Nil)
}

pub fn fetch(query: pog.Query(a), ctx: context.Context) -> Result(List(a), Nil) {
  query
  |> pog.execute(ctx.db)
  |> log.on_errorf(ctx, query_error_format)
  |> result.replace_error(Nil)
  |> result.map(fn(rows) { rows.rows })
}

pub fn fetch_one(query: pog.Query(a), ctx: context.Context) -> Result(a, Nil) {
  let result = fetch(query, ctx)
  case result {
    Error(_) -> Error(Nil)
    Ok([]) -> {
      log.error(ctx, "DB ERROR: expected one row got none")
      Error(Nil)
    }
    Ok([a]) -> Ok(a)
    Ok(_) -> {
      log.error(ctx, "DB ERROR: expected one row got many")
      Error(Nil)
    }
  }
}
