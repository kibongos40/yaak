import { defaultKeymap } from '@codemirror/commands';
import { Compartment, EditorState } from '@codemirror/state';
import { keymap, placeholder as placeholderExt, tooltips } from '@codemirror/view';
import classnames from 'classnames';
import { EditorView } from 'codemirror';
import type { MutableRefObject } from 'react';
import { useEffect, useRef } from 'react';
import { useUnmount } from 'react-use';
import './Editor.css';
import { baseExtensions, getLanguageExtension, multiLineExtensions } from './extensions';
import { singleLineExt } from './singleLine';

export interface _EditorProps {
  id?: string;
  readOnly?: boolean;
  className?: string;
  heightMode?: 'auto' | 'full';
  contentType?: string;
  autoFocus?: boolean;
  defaultValue?: string;
  placeholder?: string;
  tooltipContainer?: HTMLElement;
  useTemplating?: boolean;
  onChange?: (value: string) => void;
  onFocus?: () => void;
  singleLine?: boolean;
}

export function _Editor({
  readOnly,
  heightMode,
  contentType,
  autoFocus,
  placeholder,
  useTemplating,
  defaultValue,
  onChange,
  onFocus,
  className,
  singleLine,
}: _EditorProps) {
  const cm = useRef<{ view: EditorView; languageCompartment: Compartment } | null>(null);
  const wrapperRef = useRef<HTMLDivElement | null>(null);

  // Unmount the editor
  useUnmount(() => {
    cm.current?.view.destroy();
    cm.current = null;
  });

  // Use ref so we can update the onChange handler without re-initializing the editor
  const handleChange = useRef<_EditorProps['onChange']>(onChange);
  useEffect(() => {
    handleChange.current = onChange;
  }, [onChange]);

  // Use ref so we can update the onChange handler without re-initializing the editor
  const handleFocus = useRef<_EditorProps['onFocus']>(onFocus);
  useEffect(() => {
    handleFocus.current = onFocus;
  }, [onFocus]);

  // Update placeholder
  const placeholderCompartment = useRef(new Compartment());
  useEffect(() => {
    if (cm.current === null) return;
    const effect = placeholderCompartment.current.reconfigure(placeholderExt(placeholder ?? ''));
    cm.current?.view.dispatch({ effects: effect });
  }, [placeholder]);

  // Update language extension when contentType changes
  useEffect(() => {
    if (cm.current === null) return;
    const { view, languageCompartment } = cm.current;
    const ext = getLanguageExtension({ contentType, useTemplating });
    view.dispatch({ effects: languageCompartment.reconfigure(ext) });
  }, [contentType]);

  // Initialize the editor when ref mounts
  useEffect(() => {
    if (wrapperRef.current === null || cm.current !== null) return;
    try {
      const languageCompartment = new Compartment();
      const langExt = getLanguageExtension({ contentType, useTemplating });
      const state = EditorState.create({
        doc: `${defaultValue ?? ''}`,
        extensions: [
          languageCompartment.of(langExt),
          placeholderCompartment.current.of(placeholderExt(placeholder ?? '')),
          ...getExtensions({
            container: wrapperRef.current,
            onChange: handleChange,
            onFocus: handleFocus,
            readOnly,
            singleLine,
          }),
        ],
      });
      const view = new EditorView({ state, parent: wrapperRef.current });
      cm.current = { view, languageCompartment };
      if (autoFocus) view.focus();
    } catch (e) {
      console.log('Failed to initialize Codemirror', e);
    }
  }, [wrapperRef.current]);

  useEffect(() => {
    if (wrapperRef.current === null) return;
    syncGutterBg({ parent: wrapperRef.current, className });
  }, [className]);

  return (
    <div
      ref={wrapperRef}
      dangerouslySetInnerHTML={{ __html: '' }}
      className={classnames(
        className,
        'cm-wrapper text-base bg-gray-50',
        heightMode === 'auto' ? 'cm-auto-height' : 'cm-full-height',
        singleLine ? 'cm-singleline' : 'cm-multiline',
        readOnly && 'cm-readonly',
      )}
    />
  );
}

function getExtensions({
  container,
  readOnly,
  singleLine,
  onChange,
  onFocus,
}: Pick<_EditorProps, 'singleLine' | 'readOnly'> & {
  container: HTMLDivElement | null;
  onChange: MutableRefObject<_EditorProps['onChange']>;
  onFocus: MutableRefObject<_EditorProps['onFocus']>;
}) {
  // TODO: Ensure tooltips render inside the dialog if we are in one.
  const parent =
    container?.closest<HTMLDivElement>('[role="dialog"]') ??
    document.querySelector<HTMLDivElement>('#cm-portal') ??
    undefined;

  return [
    ...baseExtensions,
    tooltips({ parent }),
    keymap.of(singleLine ? defaultKeymap.filter((k) => k.key !== 'Enter') : defaultKeymap),
    ...(singleLine ? [singleLineExt()] : []),
    ...(!singleLine ? [multiLineExtensions] : []),
    ...(readOnly ? [EditorState.readOnly.of(true)] : []),
    ...(singleLine
      ? [
          EditorView.domEventHandlers({
            focus: (e, view) => {
              // select all text on focus, like a regular input does
              view.dispatch({ selection: { anchor: 0, head: view.state.doc.length } });
            },
            keydown: (e) => {
              // Submit nearest form on enter if there is one
              if (e.key === 'Enter') {
                const el = e.currentTarget as HTMLElement;
                const form = el.closest('form');
                form?.dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
              }
            },
          }),
        ]
      : []),

    // Handle onFocus
    EditorView.domEventHandlers({
      focus: () => {
        onFocus.current?.();
      },
    }),

    // Handle onChange
    EditorView.updateListener.of((update) => {
      if (onChange && update.docChanged) {
        onChange.current?.(update.state.doc.toString());
      }
    }),
  ];
}

const syncGutterBg = ({
  parent,
  className = '',
}: {
  parent: HTMLDivElement;
  className?: string;
}) => {
  const gutterEl = parent.querySelector<HTMLDivElement>('.cm-gutters');
  const classList = className?.split(/\s+/) ?? [];
  const bgClasses = classList
    .filter((c) => c.match(/(^|:)?bg-.+/)) // Find bg-* classes
    .map((c) => c.replace(/^bg-/, '!bg-')) // !important
    .map((c) => c.replace(/^dark:bg-/, 'dark:!bg-')); // !important
  if (gutterEl) {
    gutterEl?.classList.add(...bgClasses);
  }
};