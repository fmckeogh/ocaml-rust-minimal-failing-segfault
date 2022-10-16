let remove_duplicates l =
  let l' = List.sort Stdlib.compare l in
  let rec aux acc l =
    match (acc, l) with
    | _, [] -> List.rev acc
    | [], x :: xs -> aux [ x ] xs
    | y :: ys, x :: xs ->
        if x = y then aux (y :: ys) xs else aux (x :: y :: ys) xs
  in
  aux [] l'

let () = Callback.register "internal_util_dedup" remove_duplicates
