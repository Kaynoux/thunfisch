I/O Protokoll um mit Chess Guis zu interagieren


```mermaid
sequenceDiagram
	participant Client
	participant Engine
	
	Client --> Engine: go 
	Engine --> Client: bestmove <coords>
```

