Atue como um engenheiro sênior de compiladores, runtimes e linguagens de programação.

Analise profundamente o compilador e o runtime da nossa linguagem, atualmente implementados em C, com o objetivo de avaliar a viabilidade técnica, arquitetural e estratégica de desenvolver um backend em Rust que traduza, replique ou evolua o que já existe.

A análise deve cobrir:

1. Diagnóstico da implementação atual em C
- Arquitetura do compilador e do runtime
- Separação entre frontend, backend, runtime e bibliotecas
- Fluxo de compilação, interpretação ou execução
- Gerenciamento de memória
- Modelo de tipos
- Representação intermediária, se houver
- Tratamento de erros
- Pontos de acoplamento excessivo
- Partes frágeis, difíceis de testar ou difíceis de evoluir

2. Dificuldades encontradas na implementação em C
- Identifique quais problemas parecem decorrer da linguagem C, do modelo de memória, da ausência de abstrações seguras, do build system, da arquitetura atual ou de decisões históricas do projeto
- Analise quais partes foram difíceis de implementar e por quê
- Aponte riscos de segurança, bugs recorrentes, comportamento indefinido, complexidade desnecessária ou falta de controle semântico
- Explique onde a base atual limita a evolução da linguagem

3. Viabilidade de um backend em Rust
- Grau técnico de desenvolvimento necessário
- Complexidade estimada
- O que pode ser traduzido diretamente
- O que deveria ser redesenhado
- O que pode coexistir com C via FFI
- Riscos técnicos de migração
- Esforço necessário para atingir paridade funcional

4. Melhor approach técnico em Rust
- Proponha uma arquitetura mais clara, modular e segura
- Recomende como estruturar frontend, IR, backend, runtime, stdlib e camada de interoperabilidade
- Sugira padrões de ownership, gerenciamento de memória e APIs internas
- Proponha formas melhores de lidar com erros, escopo, tipos, módulos, lifetimes, execução e integração com código nativo
- Recomende mecanismos de controle seguro sobre a linguagem e o runtime

5. Legibilidade, acessibilidade e controle seguro da linguagem
- Avalie como tornar a implementação mais legível para mantenedores
- Como tornar a linguagem mais acessível para usuários e contribuidores
- Como melhorar mensagens de erro, diagnóstico e depuração
- Como garantir previsibilidade semântica
- Como evitar comportamento indefinido
- Como aumentar a confiança no runtime por meio de testes, validação, fuzzing e análise estática
- Como documentar decisões arquiteturais e semânticas

6. Comparação C vs Rust
Compare os dois caminhos considerando:
- Performance
- Segurança de memória
- Controle de baixo nível
- Complexidade de desenvolvimento
- Manutenibilidade
- Portabilidade
- Observabilidade
- Tooling
- Testabilidade
- Interoperabilidade
- Facilidade de evolução da linguagem

7. Estratégias de implementação
Avalie pelo menos estes caminhos:
- Manter C e apenas refatorar
- Criar um backend Rust incremental
- Reescrever partes críticas em Rust
- Criar runtime Rust separado
- Usar arquitetura híbrida C/Rust
- Reescrever totalmente compilador e runtime

Para cada estratégia, explique vantagens, desvantagens, riscos e quando ela faria sentido.

8. Plano recomendado
Proponha um plano prático com:
- Etapas de implementação
- Ordem de prioridade
- Marcos técnicos
- Critérios de sucesso
- Estratégia de testes
- Estratégia de migração
- Como validar equivalência funcional com a versão em C
- Como evitar regressões
- Quais partes devem ser atacadas primeiro

9. Conclusão
Finalize com uma recomendação objetiva:
- Vale a pena desenvolver o backend em Rust?
- Em quais condições?
- Quais problemas isso resolveria?
- Quais problemas não resolveria automaticamente?
- Quais melhorias devem ser priorizadas?
- Qual o esforço estimado: baixo, médio, alto ou muito alto?
- Qual seria o melhor approach técnico para evoluir a linguagem com segurança, legibilidade, acessibilidade e controle?


Além da análise do compilador, runtime e viabilidade de um backend em Rust, avalie também o design da linguagem em si.

Inclua uma análise crítica da sintaxe, semântica, sistema de tipos, modelo de memória, modelo de execução, tratamento de erros, diagnósticos, tooling e especificação formal.

A análise deve responder:

1. Sintaxe
- A sintaxe atual é consistente, legível e acessível?
- Existem ambiguidades gramaticais ou construções difíceis de parsear?
- Quais decisões sintáticas dificultam a implementação atual?
- Que melhorias poderiam simplificar o parser e melhorar a experiência do usuário?

2. Semântica
- As regras de escopo, mutabilidade, visibilidade, módulos, imports, chamadas de função e ordem de avaliação estão bem definidas?
- Existem comportamentos implícitos, ambíguos ou dependentes da implementação?
- Quais regras semânticas deveriam ser formalizadas antes da migração para Rust?

3. Sistema de tipos
- O sistema de tipos é estático, dinâmico ou híbrido?
- Há inferência, generics, traits/interfaces, enums, unions, nullable types ou conversões implícitas?
- Quais decisões aumentam ou reduzem segurança?
- Como tornar o sistema de tipos mais previsível, expressivo e fácil de implementar?

4. Modelo de memória
- Como valores são alocados, compartilhados e destruídos?
- Existe ownership, garbage collection, reference counting, arenas ou gerenciamento manual?
- Quais riscos existem na implementação atual em C?
- Qual modelo seria mais adequado para uma implementação segura em Rust?

5. Modelo de execução e runtime
- A linguagem compila para código nativo, bytecode, AST interpretada ou outro formato?
- O runtime é necessário em quais situações?
- Quais responsabilidades pertencem ao compilador e quais pertencem ao runtime?
- Como melhorar isolamento, segurança, controle de recursos e previsibilidade?

6. Diagnósticos e acessibilidade
- As mensagens de erro são claras, úteis e acionáveis?
- Como melhorar erros léxicos, sintáticos, semânticos e de runtime?
- Como tornar a linguagem mais acessível para iniciantes e mais produtiva para usuários avançados?

7. Especificação formal
- A linguagem possui uma especificação clara?
- Se não possuir, proponha uma estrutura de especificação contendo gramática, semântica, sistema de tipos, modelo de memória, runtime e exemplos canônicos.
- Explique quais partes precisam ser especificadas antes de desenvolver um backend em Rust.

8. Tooling e ecossistema
- Avalie a necessidade de formatter, linter, LSP, debugger, test runner, package manager, build system, documentação automática, REPL e ferramentas de benchmarking.
- Priorize quais ferramentas deveriam ser criadas primeiro.

9. Testes de conformidade
- Proponha uma suíte oficial de testes para validar a linguagem.
- Inclua testes de parser, semântica, tipos, runtime, mensagens de erro, regressão, fuzzing e equivalência entre C e Rust.

10. Roadmap de evolução
- Recomende quais mudanças devem ser feitas antes, durante e depois da criação do backend em Rust.
- Separe melhorias urgentes, melhorias estruturais e mudanças incompatíveis que podem valer a pena.