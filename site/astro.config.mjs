import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import { unified } from '@astrojs/markdown-remark';

function remarkStripLeadingH1() {
  return (tree) => {
    if (
      tree.children.length > 0 &&
      tree.children[0].type === 'heading' &&
      tree.children[0].depth === 1
    ) {
      tree.children.shift();
    }
  };
}

export default defineConfig({
  site: 'https://sovereign-account.example',
  output: 'static',
  build: {
    format: 'directory',
    assets: 'assets',
  },
  vite: { plugins: [tailwindcss()] },
  markdown: {
    processor: unified({
      remarkPlugins: [remarkStripLeadingH1],
    }),
    shikiConfig: { theme: 'github-dark' },
  },
});
