import type { Plugin } from 'unified';
import type { Root, Element, Text } from 'hast';

const DIAGRAM_MARKER = /%%\s*kipio-diagram:\s*([A-Za-z][A-Za-z0-9_-]*)/;

const TITLES: Record<string, string> = {
  Landscape: 'System Landscape',
  Context: 'System Context',
  Containers: 'Containers',
  Foundation: 'Foundation — Domain Primitives',
  Authority: 'Authority — Credential, Session, Delegation',
  Authorization: 'Authorization — Request, Validation, Replay',
  Policy: 'Policy — Decision, Effect, Consumption',
  Execution: 'Execution — Request, Context, Constraints',
  State: 'Authorization State',
  Infrastructure: 'Infrastructure — Transversal Concepts',
};

interface AnyNode {
  type: string;
  tagName?: string;
  properties?: Record<string, unknown>;
  children?: AnyNode[];
  value?: string;
}

function extractText(node: AnyNode | Text): string {
  if (node.type === 'text') return (node as Text).value;
  if (Array.isArray((node as AnyNode).children)) {
    return (node as AnyNode).children!.map(extractText).join('');
  }
  return '';
}

function el(
  tagName: string,
  properties: Record<string, unknown>,
  children: AnyNode[] = [],
): Element {
  return { type: 'element', tagName, properties, children } as unknown as Element;
}

function text(value: string): Text {
  return { type: 'text', value } as Text;
}

/**
 * Replaces ```mermaid blocks marked with %% kipio-diagram: <name> with a
 * self-contained embed block:
 *
 *   <div class="architecture-embed">
 *     <header> Title + "Open full size" link </header>
 *     <iframe src="/architecture/?diagram=<name>&introduction=false" />
 *     <footer> Interaction hints </footer>
 *   </div>
 *
 * GitHub renders the Mermaid natively (the %% comment is invisible).
 * Astro renders the interactive embed with the wrapper.
 *
 * URL uses trailing slash because static hosts 301-redirect `/path/index.html`
 * to `/path` (dropping the trailing slash). Without it, relative paths
 * inside the viewer resolve against `/` and 404.
 */
export const rehypeArchitecture: Plugin<[], Root> = () => {
  return (tree) => {
    const visit = (node: AnyNode) => {
      if (node.type === 'element' && node.tagName === 'pre') {
        const codeText = extractText(node as unknown as AnyNode);
        const match = codeText.match(DIAGRAM_MARKER);
        if (match) {
          const name = match[1];
          const title = TITLES[name] ?? name;
          const embedUrl = `/architecture/?diagram=${name}&introduction=false`;
          const fullUrl = `/architecture/?diagram=${name}`;

          const header = el(
            'div',
            { className: ['architecture-embed-header'] },
            [
              el(
                'span',
                { className: ['architecture-embed-title'] },
                [text(title)],
              ),
              el(
                'a',
                {
                  href: fullUrl,
                  target: '_blank',
                  rel: 'noopener',
                  className: ['architecture-embed-open'],
                },
                [text('Open full size ↗')],
              ),
            ],
          );

          const iframe = el('iframe', {
            src: embedUrl,
            title: `Architecture diagram: ${title}`,
            loading: 'lazy',
            allow: 'fullscreen',
          });

          const footer = el(
            'div',
            { className: ['architecture-embed-hint'] },
            [
              text('Click elements to drill down · '),
              text('Double-click background to zoom out · '),
              text('Drag to pan · Scroll to zoom · '),
              el('kbd', {}, [text('i')]),
              text(' for the diagram key'),
            ],
          );

          const wrapper = el(
            'div',
            { className: ['architecture-embed'] },
            [header, iframe, footer],
          );

          // Replace <pre> with the wrapper in the parent's children array.
          const parent = findParent(tree as unknown as AnyNode, node);
          if (parent && Array.isArray(parent.children)) {
            const idx = parent.children.indexOf(node as unknown as AnyNode);
            if (idx !== -1) {
              parent.children[idx] = wrapper as unknown as AnyNode;
            }
          }
          return;
        }
      }
      if (Array.isArray(node.children)) {
        node.children.forEach(visit);
      }
    };
    visit(tree as unknown as AnyNode);
  };
};

function findParent(
  root: AnyNode,
  target: AnyNode,
): AnyNode | null {
  if (!Array.isArray(root.children)) return null;
  for (const child of root.children) {
    if (child === target) return root;
    const found = findParent(child, target);
    if (found) return found;
  }
  return null;
}
