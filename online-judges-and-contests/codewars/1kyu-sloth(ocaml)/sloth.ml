(* Let's implement laziness!
 * A lazy value can be implemented as a closure (also known as thunk),
 * and getting a strict value out of it is just function evaluation. *)

(* Note that you should not use reference unless we explicitly ask for it.
 * Also, whenever you construct a lazy value, make sure to defer
 * all calculation as late as possible *)
module type Lazy =
sig
  type _ t
  val mk: (unit -> 'a) -> 'a t
  val get: 'a t -> 'a
  val map: ('a -> 'b) -> 'a t -> 'b t
  val return: 'a -> 'a t
  val join: 'a t t -> 'a t
  val bind:  ('a -> 'b t) -> 'a t -> 'b t
  val (>>=): 'a t -> ('a -> 'b t) -> 'b t
  (* The argument is a lazy value that might refer to itself.
   * Tie the value to itself using reference. *)
  val tie: ('a t -> 'a t) -> 'a t
end;;

module type LazyMin =
sig
  type _ t
  val mk: (unit -> 'a) -> 'a t
  val get: 'a t -> 'a
end;;

module LazyImpl (LM: LazyMin): Lazy with type 'a t = 'a LM.t =
struct
  include LM
  let map f a = mk (fun _ -> f (get a))
  let return a = mk (fun _ -> a)
  let join a = mk (fun _ -> get (get a))
  let bind m a = join (map m a)
  let (>>=) a m = bind m a
  let tie f =
    let tem = ref (mk (fun _ -> failwith "uninitialize")) in
      tem := mk (fun _ -> get (f (!tem))); !tem
end;;

module LazyThunk: Lazy with type 'a t = unit -> 'a = LazyImpl(
struct
  type 'a t = unit -> 'a
  let mk f = f
  let get f = f ()
end);;

(* each time the ‘get’ function is used, the thunk has to be evaluated *)
module LazyOption:
  Lazy with type 'a t = 'a option ref * (unit -> 'a) = LazyImpl(
struct
  type 'a t = 'a option ref * (unit -> 'a)
  let mk f = (ref None, f)
  let get (o, f) =
    match !o with
      None -> let ret = f () in o := Some ret; ret
    | Some x -> x
end);;

(* two components for a Lazy: a thunk and a cache.
 * cache the thunk, no 2 components! (tagless) *)
module LazyTagless: Lazy with type 'a t = (unit -> 'a) ref = LazyImpl(
struct
  type 'a t = (unit -> 'a) ref
  let mk = ref
  let get x = let ret = (!x)() in x := (fun _ -> ret); ret
end);;
(* Notice how most definition of lazy can be derived from other?
 * Try to lookup how module works and refactor them. *)

(* To test that implementation work, do some infinite stream *)
module type StreamSig =
sig
  module L: Lazy
  type 'a stream = Stream of ('a * 'a stream) L.t
  val mk: (unit -> 'a * 'a stream) -> 'a stream
  val hd: 'a stream -> 'a
  val tl: 'a stream -> 'a stream
  val gen: ('a -> 'a) -> 'a L.t -> 'a stream
  val map: ('a -> 'b) -> 'a stream -> 'b stream
  val zip: 'a stream -> 'b stream -> ('a * 'b) stream
  val zipWith: ('a * 'b -> 'c) -> 'a stream -> 'b stream -> 'c stream
  val takeWhile: ('a -> bool) -> 'a stream -> 'a list
  val app: 'a list -> 'a stream -> 'a stream
  val fib_aux: int stream -> int stream 
  val join: 'a stream stream -> 'a stream
end;;

module Stream (L: Lazy): StreamSig with module L = L =
struct
  module L = L
  type 'a stream = Stream of ('a * 'a stream) L.t
  let rec mk f = Stream (L.mk f)
  let hd (Stream x) = fst (L.get x)
  let tl (Stream x) = mk (fun _ ->
    match snd (L.get x) with Stream s -> L.get s)
  let rec gen f a = mk (fun _ ->
    (L.get a, gen f (L.mk (fun _ -> f (L.get a)))))
  let rec map f x = mk (fun _ -> (f (hd x), map f (tl x)))
  let rec zip l r = mk (fun _ -> ((hd l, hd r), zip (tl l) (tl r)))
  let zipWith f l r = map f (zip l r)
  let rec takeWhile p x = if p (hd x) then hd x :: takeWhile p (tl x) else []
  let rec app = function
    [] -> (fun x -> x)
  | (a :: ax) -> (fun x -> mk (fun _ -> (a, (app ax x))))
  let fib_aux self =
    app [0; 1] (zipWith (fun (l, r) -> l + r) self (tl self))
  let join ss =
    let rec fold l s =
      mk (fun () -> L.get (match (app (List.map hd l) (fold (hd s :: List.map tl l) (tl s))) with
        Stream s -> s)) 
    in fold [] ss
end;;
