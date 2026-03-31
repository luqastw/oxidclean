# Solução de Problemas

## Problemas Comuns

### "Banco de dados do Pacman não encontrado"

**Erro:**
```
Banco de dados do Pacman não encontrado ou corrompido em: /var/lib/pacman/local
```

**Causa:** O OxidClean não consegue acessar o banco de dados do pacman.

**Soluções:**
1. Verifique se você está em um sistema Arch Linux ou derivado
2. Verifique as permissões:
   ```bash
   ls -la /var/lib/pacman/local
   ```
3. Sincronize o banco de dados:
   ```bash
   sudo pacman -Sy
   ```

### "Permissões insuficientes"

**Erro:**
```
Permissões insuficientes: Operação requer permissões de root
```

**Causa:** Comandos de remoção precisam de root.

**Solução:**
```bash
sudo oxidclean clean
```

### "Sistema não suportado"

**Erro:**
```
Sistema não suportado: esperado Arch Linux ou derivado
```

**Causa:** O OxidClean detectou que o sistema não é Arch Linux.

**Soluções:**
1. Verifique se `/etc/os-release` existe
2. Confirme que é um sistema baseado em Arch:
   ```bash
   cat /etc/os-release | grep -i arch
   ```

### Scan está muito lento

**Causa:** Primeira execução ou cache expirado.

**Soluções:**
1. O cache dura 5 minutos - execuções subsequentes são mais rápidas
2. Verifique se o disco não está lento
3. Para muitos pacotes (>3000), é normal levar alguns segundos

### Pacote removido mas ainda aparece como órfão

**Causa:** O cache do scan está desatualizado.

**Solução:**
```bash
# Força um novo scan (aguarde o cache expirar ou reinicie)
# Ou simplesmente execute scan novamente após alguns minutos
oxidclean scan
```

## FAQ

### O OxidClean pode danificar meu sistema?

O OxidClean tem várias proteções:
- Pacotes críticos são protegidos (base, linux, pacman, etc.)
- Modo interativo pede confirmação para cada remoção
- Modo dry-run permite simular antes de executar
- Todas as operações são registradas em log

### Qual a diferença entre OxidClean e `pacman -Qdtq`?

O OxidClean oferece funcionalidades adicionais:
- Classificação de risco por pacote
- Análise de dependências completa
- Gerenciamento de cache
- Interface visual com cores e tabelas
- Proteção de pacotes críticos
- Logging de operações

### Posso usar em Manjaro/EndeavourOS?

Sim! O OxidClean funciona em qualquer distribuição baseada em Arch Linux.

### Como faço backup antes de limpar?

```bash
# Listar pacotes instalados explicitamente
pacman -Qqe > packages-explicit.txt

# Listar todos os pacotes
pacman -Qq > packages-all.txt

# Exportar relatório
oxidclean scan --export backup-report.json
```

### O cache do pacman está ocupando muito espaço

Use o comando cache:
```bash
# Ver estatísticas
oxidclean cache --stats

# Limpar mantendo apenas 1 versão
sudo oxidclean cache --clean --keep 1

# Para limpeza mais agressiva, use paccache diretamente
sudo paccache -rk1
```

## Logs e Debug

### Localização dos logs

```bash
# Log de operações
~/.config/oxidclean/history.log

# Configuração
~/.config/oxidclean/config.toml
```

### Modo verbose

Para mais informações durante execução:
```bash
oxidclean -v scan
```

### Reportar bugs

Se encontrar um bug:
1. Execute com `--verbose` e capture a saída
2. Inclua informações do sistema:
   ```bash
   uname -a
   cat /etc/os-release
   pacman --version
   ```
3. Abra uma issue no repositório do projeto
