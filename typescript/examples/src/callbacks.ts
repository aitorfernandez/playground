type CallbackFn = (text: string) => void;

const myCallback = (text: string) => {
	console.log(`myCallback called with ${text}`);
};

function withCallback(message: string, callbackFn: CallbackFn) {
	callbackFn(`${message} from withCallback`);
}

withCallback("text", myCallback);
