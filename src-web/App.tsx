import classnames from 'classnames';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Button } from './components/Button';
import { Divider } from './components/Divider';
import { Grid } from './components/Grid';
import { RequestPane } from './components/RequestPane';
import { ResponsePane } from './components/ResponsePane';
import { Sidebar } from './components/Sidebar';
import { HStack } from './components/Stacks';
import {
  useDeleteRequest,
  useRequests,
  useRequestUpdate,
  useSendRequest,
} from './hooks/useRequest';

type Params = {
  workspaceId: string;
  requestId?: string;
};

function App() {
  const p = useParams<Params>();
  const workspaceId = p.workspaceId ?? '';
  const { data: requests } = useRequests(workspaceId);
  const request = requests?.find((r) => r.id === p.requestId);

  const [screenWidth, setScreenWidth] = useState(window.innerWidth);
  useEffect(() => {
    window.addEventListener('resize', () => setScreenWidth(window.innerWidth));
  }, []);
  const isH = screenWidth > 900;

  return (
    <div className="grid grid-cols-[auto_1fr] h-full text-gray-900">
      <Sidebar requests={requests ?? []} workspaceId={workspaceId} activeRequestId={request?.id} />
      {request && (
        <div className="p-2 h-full">
          <div className="grid grid-rows-[auto_1fr] rounded-md h-full overflow-hidden">
            <HStack
              data-tauri-drag-region
              className="h-10 px-3 bg-gray-50"
              justify="center"
              items="center"
            >
              {request.name}
            </HStack>
            <div
              className={classnames(
                'py-2 px-1 bg-gray-25 grid overflow-auto',
                isH ? 'grid-cols-[1fr_1fr]' : 'grid-rows-[minmax(0,auto)_minmax(0,100%)]',
              )}
            >
              <RequestPane
                fullHeight={isH}
                request={request}
                className={classnames(
                  'border-gray-100/50',
                  isH ? 'pr-0 border-r' : 'pb-3 mb-1 border-b',
                )}
              />
              <ResponsePane requestId={request.id} />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
