/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />

self.addEventListener('notificationclick', (event: Event) => {
  const notificationEvent = event as NotificationEvent;
  console.log('Notification clicked!', notificationEvent.action); // Debug log
  notificationEvent.notification.close();

  // Only handle if main body or 'open' action is clicked
  if (!notificationEvent.action || notificationEvent.action === 'open') {
    const urlToOpen = notificationEvent.notification.data?.url || '/LogView';
    const swSelf = self as unknown as ServiceWorkerGlobalScope;
    notificationEvent.waitUntil(
      swSelf.clients.matchAll({ type: 'window', includeUncontrolled: true }).then((clientList: readonly WindowClient[]) => {
        for (const client of clientList) {
          if (client.url.includes(urlToOpen) && 'focus' in client) {
            return client.focus();
          }
        }
        if ('openWindow' in swSelf.clients) {
          return swSelf.clients.openWindow!(urlToOpen);
        }
      })
    );
  }
});