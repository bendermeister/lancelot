import backend/context
import backend/log
import gleam/result
import wisp

pub fn handle_request(ctx: context.Context, req: wisp.Request) -> wisp.Response {
  use ctx, req <- middleware(ctx, req)

  wisp.ok()
  |> wisp.string_body("Hello World")
  |> Ok
}

pub fn middleware(
  ctx: context.Context,
  req: wisp.Request,
  callback: fn(context.Context, wisp.Request) -> Result(wisp.Response, Nil),
) -> wisp.Response {
  log.info(ctx, "got request: " <> req.path)

  callback(ctx, req)
  |> log.on_ok(ctx, "request handled successfully")
  |> log.on_error(ctx, "error during request handling")
  |> result.unwrap(wisp.internal_server_error())
}
