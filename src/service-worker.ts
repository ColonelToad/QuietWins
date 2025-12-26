/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />

declare let self: ServiceWorkerGlobalScope;

self.addEventListener('notificationclick', (event) => {
  console.log('Notification clicked!'); // Debug log
  event.notification.close();
  
  const urlToOpen = event.notification.data?.url || '/LogView';
  
  event.waitUntil(
    self.clients.matchAll({ type: 'window', includeUncontrolled: true })
      .then((clientList) => {
        for (const client of clientList) {
          if (client.url.includes(urlToOpen) && 'focus' in client) {
            return client.focus();
          }
        }
        if (self.clients.openWindow) {
          return self.clients.openWindow(urlToOpen);
        }
      })
  );
});