import { writable } from 'svelte/store';

type AppState = {
    history: Array<ServerMessage>,
};

function createGlobalState() {
    const { subscribe, set, update } = writable<{
        data: AppState;
        connected: boolean;
    }>({
        data: { history: [] } as AppState,
        connected: false,
    });

    let socket: WebSocket;

    function connect() {
        const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
        const url = `${protocol}://${location.host}/ws`;
        socket = new WebSocket(url);

        socket.onopen = () => {
            update(s => ({ ...s, connected: true }));
        };

        socket.onmessage = (event) => {
            try {
                const msg = JSON.parse(event.data);

                update(s => {
                    s.data.history.push(msg);

                    return {
                        ...s,
                        connected: true,
                    };
                });
            } catch (e) {
                console.log("Error parsing message:", e);
            }
        };

        socket.onclose = () => {
            update(s => ({ ...s, connected: false }));
            setTimeout(connect, 3000);
        };
    }

    if (typeof window !== 'undefined') {
        connect();
    }

    return {
        subscribe,
        send: (msg: string) => socket?.send(msg)
    };
}

export const appState = createGlobalState();
