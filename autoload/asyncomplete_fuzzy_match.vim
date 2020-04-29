let s:job_id = -1
let s:buffer = ''
let s:complete_options_queue = []

function! asyncomplete_fuzzy_match#start() abort
  call s:start_process()
  let g:asyncomplete_preprocessor = [function('asyncomplete_fuzzy_match#preprocessor')]
endfunction

function! s:start_process() abort
  if s:job_id < 0 && executable(g:asyncomplete_fuzzy_match_path)
    let s:job_id = async#job#start([g:asyncomplete_fuzzy_match_path], {
      \   'on_stdout': { _, data -> s:handle_response(data) },
      \   'on_exit': { -> s:handle_exit() },
      \ })
  endif
endfunction

function! asyncomplete_fuzzy_match#preprocessor(options, matches) abort
  if s:job_id >= 0
    let l:completions = {
      \   'pattern': a:options.base,
      \   'lists': [],
      \ }
    for [l:source_name, l:matches] in items(a:matches)
      call add(l:completions.lists, {
        \   'items': l:matches.items,
        \   'priority': get(g:asyncomplete_fuzzy_match_priorities, l:source_name, 0),
        \ })
    endfor
    call add(s:complete_options_queue, a:options)
    call async#job#send(s:job_id, json_encode(l:completions) . "\n")
  else
    let l:items = []
    for [l:source_name, l:matches] in items(a:matches)
      for l:item in l:matches['items']
        if stridx(l:item['word'], a:options['base']) == 0
          call add(l:items, l:item)
        endif
      endfor
    endfor
    call asyncomplete#preprocess_complete(a:options, l:items)
  endif
endfunction

function! s:handle_response(data) abort
  for l:chunk in a:data
    if l:chunk !=# ''
      let s:buffer .= l:chunk
    else
      if !empty(s:complete_options_queue)
        let l:complete_options = remove(s:complete_options_queue, 0)
        if s:buffer !=# ''
          call asyncomplete#preprocess_complete(l:complete_options, json_decode(s:buffer))
        endif
      endif
      let s:buffer = ''
    endif
  endfor
endfunction

function! s:handle_exit() abort
  let s:job_id = -1
  let s:buffer = ''
  let s:complete_options_queue = []
endfunction

" vim: set ts=2 sts=2 sw=2 :
