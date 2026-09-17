import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import { unified } from '@astrojs/markdown-remark';

/**
 * Removes the leading H1 from markdown bodies.
 * The page layout renders its own H1 from the frontmatter title.
 * The raw .md files copied to dist/ keep their H1 for standalone consumption.
 */
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
  },
  vite: {
    plugins: [tailwindcss()],
  },
  markdown: {
    processor: unified({
      remarkPlugins: [remarkStripLeadingH1],
    }),
    shikiConfig: {
      theme: 'github-dark',
    },
  },
});
