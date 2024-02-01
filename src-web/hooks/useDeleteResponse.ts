import { useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api';
import { trackEvent } from '../lib/analytics';
import type { HttpResponse } from '../lib/models';
import { responsesQueryKey } from './useResponses';

export function useDeleteResponse(id: string | null) {
  const queryClient = useQueryClient();
  return useMutation<HttpResponse>({
    mutationFn: async () => {
      return await invoke('cmd_delete_response', { id: id });
    },
    onSettled: () => trackEvent('HttpResponse', 'Delete'),
    onSuccess: ({ requestId, id: responseId }) => {
      queryClient.setQueryData<HttpResponse[]>(responsesQueryKey({ requestId }), (responses) =>
        (responses ?? []).filter((response) => response.id !== responseId),
      );
    },
  });
}
