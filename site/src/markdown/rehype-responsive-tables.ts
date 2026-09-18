import type { Plugin } from 'unified';
import type { Root, Element } from 'hast';

/**
 * Wraps every <table> in a scrollable container so wide tables do not
 * overflow on narrow viewports.
 *
 * The wrapper <div class="table-scroll"> is styled in globals.css.
 */
export const rehypeResponsiveTables: Plugin<[], Root> = () => {
  return (tree) => {
    const visit = (node: Root | Element) => {
      if (!Array.isArray(node.children)) return;

      for (let i = 0; i < node.children.length; i++) {
        const child = node.children[i];
        if (child.type === 'element' && child.tagName === 'table') {
          const wrapper: Element = {
            type: 'element',
            tagName: 'div',
            properties: {
              className: ['table-scroll'],
            },
            children: [child],
          };
          node.children[i] = wrapper;
        } else {
          visit(child as Element);
        }
      }
    };

    visit(tree);
  };
};
