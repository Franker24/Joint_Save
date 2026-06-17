# Guía de Contribución (Contributing Guide)

¡Gracias por tu interés en colaborar con JointSave! Para mantener una base de código limpia, consistente y libre de errores, seguimos pautas estrictas de formato y calidad de código.

## 🛠️ Calidad de Código y Estilo

Antes de confirmar (commit) tus cambios o enviar un Pull Request (PR), debés asegurarte de que tu código cumpla con los estándares de estilo configurados mediante **ESLint** y **Prettier**.

### Scripts del Frontend

Dentro del directorio `frontend/`, disponés de los siguientes comandos para validar y formatear el código:

* **Verificar el estilo (Linting):**
  ```bash
  npm run lint
  ```
  Este comando analiza el código TypeScript y React en busca de malas prácticas y errores potenciales (por ejemplo, llamadas incorrectas de Hooks o el uso no permitido de `any`).

* **Corregir automáticamente errores de estilo (Lint Fix):**
  ```bash
  npm run lint:fix
  ```
  Corrige automáticamente los problemas de ESLint que tengan solución directa.

* **Verificar el formato (Prettier Check):**
  ```bash
  npm run format:check
  ```
  Valida que todos los archivos sigan el formato y las reglas de diseño definidas en el proyecto.

* **Aplicar el formato automáticamente (Prettier Format):**
  ```bash
  npm run format
  ```
  Aplica el formato correcto a todos los archivos del frontend de forma automática.

---

## 🚀 Validación en Integración Continua (CI)

Cada Pull Request enviado al repositorio ejecutará automáticamente de forma obligatoria los siguientes checks en el pipeline de GitHub Actions:
1. `npm run lint`
2. `npm run format:check`

Si alguno de estos comandos falla, **el pipeline de CI se romperá y bloqueará la integración del PR**. Por lo tanto, recordá siempre correr estos comandos localmente antes de subir tus commits.

¡Gracias por ayudarnos a mantener la calidad del proyecto!
