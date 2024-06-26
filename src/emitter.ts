import mitt, { Emitter } from 'mitt';

type Events = {
  error: string;
};

export const emitter: Emitter<Events>  = mitt<Events>();
