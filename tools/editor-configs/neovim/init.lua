-- Zenith LSP starter config for Neovim 0.10+.
--
-- Put this in your Neovim config, then adjust zenith_lsp_path if needed.
-- Example:
--   vim.g.zenith_lsp_path = "C:/path/to/zt-lsp.exe"

vim.filetype.add({
  extension = {
    zt = "zenith",
  },
})

vim.api.nvim_create_autocmd({ "BufReadPost", "BufNewFile" }, {
  pattern = "*.zt",
  callback = function(event)
    local root = vim.fs.root(event.buf, { "zenith.ztproj", ".git" }) or vim.fn.getcwd()
    local command = vim.g.zenith_lsp_path or "zt-lsp"

    vim.lsp.start({
      name = "zenith",
      cmd = { command },
      root_dir = root,
    })
  end,
})
