#!/usr/bin/env python

import SimpleHTTPServer
import SocketServer

PORT = 4000

class Handler(SimpleHTTPServer.SimpleHTTPRequestHandler):
    def guess_type(self, path):
        mimetype = SimpleHTTPServer.SimpleHTTPRequestHandler.guess_type(self, path)
        if mimetype == 'application/octet-stream':
            if path.endswith('wasm'):
                mimetype = 'application/wasm'

        return mimetype

Handler.extensions_map['wasm'] = 'application/wasm'

httpd = SocketServer.TCPServer(('', PORT), Handler)

print('Serving at ', PORT)
httpd.serve_forever()
