-- Subliminal Nightfall colorscheme for Neovim
-- Maintainer: Mike Hamrah
-- Based on the Zed theme

vim.cmd('highlight clear')
if vim.fn.exists('syntax_on') then
  vim.cmd('syntax reset')
end

vim.g.colors_name = 'subliminal-nightfall'
vim.o.background = 'dark'

-- Color palette
local colors = {
  bg = '#191724',
  bg_alt = '#1f1d2e',
  bg_highlight = '#26233a',
  bg_line = '#2e3239',
  
  fg = '#e0def4',
  fg_alt = '#d4d4d4',
  fg_muted = '#a0a0a0',
  fg_dim = '#7f7f7f',
  
  border = '#484e5b',
  border_alt = '#363b45',
  
  -- Syntax colors
  red = '#bf616a',
  green = '#a9cfa4',
  yellow = '#ffe2a9',
  blue = '#6699cc',
  magenta = '#f1a5ab',
  cyan = '#5fb3b3',
  purple = '#c4a7e7',
  teal = '#9ccfd8',
  dark_blue = '#31748f',
  
  -- UI colors
  selection = '#484e5b',
  search = '#6699cc80',
  line_number = '#8a9099',
  line_number_active = '#c7c7c7',
  
  -- Git colors
  git_add = '#a9cfa4',
  git_change = '#ffe2a9',
  git_delete = '#bf616a',
  
  -- Diagnostic colors
  error = '#bf616a',
  warning = '#ffe2a9',
  info = '#6699cc',
  hint = '#a0a0a0',
}

-- Helper function to set highlights
local function hl(group, opts)
  vim.api.nvim_set_hl(0, group, opts)
end

-- Editor highlights
hl('Normal', { fg = colors.fg, bg = colors.bg })
hl('NormalFloat', { fg = colors.fg, bg = colors.bg_alt })
hl('NormalNC', { fg = colors.fg, bg = colors.bg })
hl('LineNr', { fg = colors.line_number })
hl('CursorLineNr', { fg = colors.line_number_active })
hl('CursorLine', { bg = colors.bg_line })
hl('ColorColumn', { bg = colors.bg_line })
hl('Cursor', { fg = colors.bg, bg = colors.fg })
hl('TermCursor', { fg = colors.bg, bg = colors.fg })
hl('Visual', { bg = colors.selection, fg = colors.fg })
hl('VisualNOS', { bg = colors.selection })
hl('Search', { bg = colors.search })
hl('IncSearch', { bg = colors.cyan, fg = colors.bg })
hl('CurSearch', { bg = colors.cyan, fg = colors.bg })
hl('MatchParen', { bg = colors.selection })

-- Window/UI elements
hl('SignColumn', { bg = colors.bg_alt })
hl('Folded', { fg = colors.fg_muted, bg = colors.bg_line })
hl('FoldColumn', { fg = colors.fg_dim, bg = colors.bg_alt })
hl('VertSplit', { fg = colors.border })
hl('WinSeparator', { fg = colors.border })
hl('StatusLine', { fg = colors.fg, bg = colors.bg })
hl('StatusLineNC', { fg = colors.fg_muted, bg = colors.bg_alt })
hl('TabLine', { fg = colors.fg_muted, bg = colors.bg_alt })
hl('TabLineFill', { bg = colors.bg_alt })
hl('TabLineSel', { fg = colors.fg, bg = colors.bg })
hl('Pmenu', { fg = colors.fg, bg = colors.bg_alt })
hl('PmenuSel', { fg = colors.fg, bg = colors.bg_highlight })
hl('PmenuSbar', { bg = colors.bg_line })
hl('PmenuThumb', { bg = colors.border })

-- Syntax highlighting
hl('Comment', { fg = colors.fg_dim, italic = true })
hl('Constant', { fg = colors.magenta })
hl('String', { fg = colors.teal })
hl('Character', { fg = colors.teal })
hl('Number', { fg = colors.purple })
hl('Boolean', { fg = colors.magenta })
hl('Float', { fg = colors.purple })

hl('Identifier', { fg = colors.fg })
hl('Function', { fg = colors.teal })

hl('Statement', { fg = colors.dark_blue })
hl('Conditional', { fg = colors.dark_blue })
hl('Repeat', { fg = colors.dark_blue })
hl('Label', { fg = colors.blue })
hl('Operator', { fg = colors.cyan })
hl('Keyword', { fg = colors.dark_blue })
hl('Exception', { fg = colors.dark_blue })

hl('PreProc', { fg = colors.magenta })
hl('Include', { fg = colors.magenta })
hl('Define', { fg = colors.magenta })
hl('Macro', { fg = colors.magenta, bold = true })
hl('PreCondit', { fg = colors.magenta })

hl('Type', { fg = colors.dark_blue })
hl('StorageClass', { fg = colors.dark_blue })
hl('Structure', { fg = colors.purple })
hl('Typedef', { fg = colors.dark_blue })

hl('Special', { fg = colors.cyan })
hl('SpecialChar', { fg = colors.teal })
hl('Tag', { fg = colors.purple })
hl('Delimiter', { fg = colors.fg_muted })
hl('SpecialComment', { fg = colors.fg_muted, italic = true })
hl('Debug', { fg = colors.red })

hl('Underlined', { underline = true })
hl('Ignore', { fg = colors.fg_dim })
hl('Error', { fg = colors.error })
hl('Todo', { fg = colors.yellow, bold = true })

-- Treesitter highlights
hl('@attribute', { fg = colors.magenta, italic = true })
hl('@boolean', { fg = colors.magenta })
hl('@character', { fg = colors.teal })
hl('@comment', { fg = colors.fg_dim, italic = true })
hl('@comment.documentation', { fg = colors.fg_muted, italic = true })
hl('@conditional', { fg = colors.dark_blue })
hl('@constant', { fg = colors.magenta })
hl('@constant.builtin', { fg = colors.magenta })
hl('@constant.macro', { fg = colors.magenta })
hl('@constructor', { fg = colors.purple })
hl('@decorator', { fg = colors.magenta, italic = true })
hl('@exception', { fg = colors.dark_blue })
hl('@field', { fg = colors.fg })
hl('@float', { fg = colors.purple })
hl('@function', { fg = colors.teal })
hl('@function.builtin', { fg = colors.teal, bold = true })
hl('@function.macro', { fg = colors.magenta, bold = true })
hl('@function.method', { fg = colors.teal })
hl('@include', { fg = colors.magenta })
hl('@keyword', { fg = colors.dark_blue })
hl('@keyword.function', { fg = colors.dark_blue })
hl('@keyword.operator', { fg = colors.cyan })
hl('@label', { fg = colors.blue })
hl('@namespace', { fg = colors.purple })
hl('@number', { fg = colors.purple })
hl('@operator', { fg = colors.cyan })
hl('@parameter', { fg = colors.fg_muted, italic = true })
hl('@property', { fg = colors.fg })
hl('@punctuation.bracket', { fg = colors.fg_muted })
hl('@punctuation.delimiter', { fg = colors.fg_muted })
hl('@punctuation.special', { fg = colors.fg_muted })
hl('@repeat', { fg = colors.dark_blue })
hl('@string', { fg = colors.teal })
hl('@string.escape', { fg = colors.teal })
hl('@string.regex', { fg = colors.teal })
hl('@string.special', { fg = colors.teal })
hl('@tag', { fg = colors.purple })
hl('@tag.attribute', { fg = colors.magenta, italic = true })
hl('@tag.delimiter', { fg = colors.fg_muted })
hl('@text.emphasis', { fg = colors.magenta, italic = true })
hl('@text.literal', { fg = colors.purple })
hl('@text.strong', { fg = colors.magenta, bold = true })
hl('@text.title', { fg = colors.blue, bold = true })
hl('@text.uri', { fg = colors.cyan })
hl('@type', { fg = colors.dark_blue })
hl('@type.builtin', { fg = colors.dark_blue })
hl('@type.definition', { fg = colors.dark_blue })
hl('@variable', { fg = colors.fg })
hl('@variable.builtin', { fg = colors.cyan, italic = true })

-- LSP semantic tokens
hl('@lsp.type.class', { fg = colors.purple })
hl('@lsp.type.decorator', { fg = colors.magenta, italic = true })
hl('@lsp.type.enum', { fg = colors.purple })
hl('@lsp.type.enumMember', { fg = colors.magenta })
hl('@lsp.type.function', { fg = colors.teal })
hl('@lsp.type.interface', { fg = colors.dark_blue, bold = true })
hl('@lsp.type.macro', { fg = colors.magenta, bold = true })
hl('@lsp.type.method', { fg = colors.teal })
hl('@lsp.type.namespace', { fg = colors.purple })
hl('@lsp.type.parameter', { fg = colors.fg_muted, italic = true })
hl('@lsp.type.property', { fg = colors.fg })
hl('@lsp.type.struct', { fg = colors.purple })
hl('@lsp.type.type', { fg = colors.dark_blue })
hl('@lsp.type.typeParameter', { fg = colors.purple, italic = true })
hl('@lsp.type.variable', { fg = colors.fg })

-- Diagnostics
hl('DiagnosticError', { fg = colors.error })
hl('DiagnosticWarn', { fg = colors.warning })
hl('DiagnosticInfo', { fg = colors.info })
hl('DiagnosticHint', { fg = colors.hint })
hl('DiagnosticUnderlineError', { sp = colors.error, undercurl = true })
hl('DiagnosticUnderlineWarn', { sp = colors.warning, undercurl = true })
hl('DiagnosticUnderlineInfo', { sp = colors.info, undercurl = true })
hl('DiagnosticUnderlineHint', { sp = colors.hint, undercurl = true })

-- Git signs
hl('GitSignsAdd', { fg = colors.git_add })
hl('GitSignsChange', { fg = colors.git_change })
hl('GitSignsDelete', { fg = colors.git_delete })

-- Telescope
hl('TelescopeBorder', { fg = colors.border })
hl('TelescopeSelection', { bg = colors.bg_highlight })
hl('TelescopeMatching', { fg = colors.cyan })

-- NvimTree
hl('NvimTreeNormal', { fg = colors.fg, bg = colors.bg_alt })
hl('NvimTreeFolderName', { fg = colors.fg })
hl('NvimTreeFolderIcon', { fg = colors.blue })
hl('NvimTreeRootFolder', { fg = colors.cyan, bold = true })
hl('NvimTreeGitDirty', { fg = colors.git_change })
hl('NvimTreeGitNew', { fg = colors.git_add })
hl('NvimTreeGitDeleted', { fg = colors.git_delete })

-- WhichKey
hl('WhichKey', { fg = colors.cyan })
hl('WhichKeyGroup', { fg = colors.blue })
hl('WhichKeyDesc', { fg = colors.fg })
hl('WhichKeySeparator', { fg = colors.fg_dim })

-- Diff
hl('DiffAdd', { bg = '#a9cfa433', fg = colors.git_add })
hl('DiffChange', { bg = '#ffe2a933', fg = colors.git_change })
hl('DiffDelete', { bg = '#bf616a33', fg = colors.git_delete })
hl('DiffText', { bg = '#6699cc1a', fg = colors.blue })
