import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import { unified } from '@astrojs/markdown-remark';
import { rehypeArchitecture } from './src/markdown/rehype-architecture.ts';
import { rehypeResponsiveTables } from './src/markdown/rehype-responsive-tables.ts';

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
      rehypePlugins: [rehypeArchitecture, rehypeResponsiveTables],
    }),
    shikiConfig: { theme: 'github-dark' },
  },
});
