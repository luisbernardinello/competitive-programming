class Delayed
  constructor: (id1, args1) ->
    @id = id1
    @args = args1

tco = (fd) ->
  fns = {}
  fd.map ([id, args, body]) ->
    fns[id] = new Function(args..., body)
    global[id] = (args...) ->
      new Delayed(id, args)
    (args...) ->
      d = fns[id](args...)
      while d instanceof Delayed
        d = fns[d.id](d.args...)
      d