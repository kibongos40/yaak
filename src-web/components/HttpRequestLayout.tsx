import type { CSSProperties } from 'react';
import React from 'react';
import type { HttpRequest } from '../lib/models';
import { SplitLayout } from './core/SplitLayout';
import { RequestPane } from './RequestPane';
import { ResponsePane } from './ResponsePane';

interface Props {
  activeRequest: HttpRequest;
  style: CSSProperties;
}

export function HttpRequestLayout({ activeRequest, style }: Props) {
  return (
    <SplitLayout
      name="http_layout"
      className="p-3 gap-1.5"
      style={style}
      firstSlot={({ orientation, style }) => (
        <RequestPane
          style={style}
          activeRequest={activeRequest}
          fullHeight={orientation === 'horizontal'}
        />
      )}
      secondSlot={({ style }) => <ResponsePane activeRequest={activeRequest} style={style} />}
    />
  );
}