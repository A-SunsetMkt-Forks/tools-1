isNaN({});

(isNaN)({});

globalThis.isNaN({});

(globalThis).isNaN({});

globalThis["isNaN"]({});

(globalThis)[("isNaN")]({});

function localIsNaN(isNaN) {
    globalThis.isNaN({});
}

localIsNaN(isNaN);
