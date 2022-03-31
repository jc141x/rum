import cookie from 'cookie';
import { v4 as uuid } from '@lukeed/uuid';

export const handle = async ({ event, resolve }) => {
  // const cookies = cookie.parse(event.request.headers.cookie || '');
  // event.request.locals.userid = cookies.userid || uuid();

  // TODO https://github.com/sveltejs/kit/issues/1046
  // if (event.request.query.has('_method')) {
  //   event.request.method = request.query.get('_method').toUpperCase();
  // }

  const response = await resolve(event, {ssr: false});

  // if (!cookies.userid) {
  //   // if this is the first time the user has visited this app,
  //   // set a cookie so that we recognise them when they return
  //   response.headers['set-cookie'] = `userid=${revent.equest.locals.userid}; Path=/; HttpOnly`;
  // }

  return response;
};
