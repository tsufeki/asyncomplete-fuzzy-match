if exists('g:asyncomplete_fuzzy_match_loaded')
    finish
endif
let g:asyncomplete_fuzzy_match_loaded = 1

let g:asyncomplete_fuzzy_match_path = get(g:, 'asyncomplete_fuzzy_match_path', expand('<sfile>:p:h:h') . '/target/release/asyncomplete-fuzzy-matcher')
let g:asyncomplete_fuzzy_match_priorities = get(g:, 'asyncomplete_fuzzy_match_priorities', {})

augroup asyncomplete_fuzzy_match
    autocmd!
    autocmd User asyncomplete_setup call asyncomplete_fuzzy_match#start()
augroup END
