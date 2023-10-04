// document.addEventListener("DOMContentLoaded", function () {
//     const accessToken = "Z2l0aHViX3BhdF8xMUFVVlFOQ1kwc2hYTklEc3N1b1dYX1RxUHNiOTBaVFB3RTFiUUpJMW1uY1NPSmdDWTFxM2V6MFVJaXIwVU5lY3NITVhHVjRFUGl6UTNWTXNq";
//     const repoInfoElement = document.getElementById("repoInfo");
  
//     // GitHub API를 사용하여 마지막으로 수정된 레포지토리 가져오기
//     async function fetchLastUpdatedRepository() {
//       try {
//         const response = await fetch("https://api.github.com/user/repos?sort=updated&per_page=1&page=1", {
//           headers: {
//             Authorization: `token ${atob(accessToken)}`,
//           },
//         });
  
//         if (response.ok) {
//           const data = await response.json();
//           if (data.length > 0) {
//             const lastUpdatedRepo = data[0];
//             console.log(lastUpdatedRepo)
//             repoInfoElement.innerHTML = `
//               <p><strong>이름:</strong> ${lastUpdatedRepo.name}</p>
//               <p><strong>소유자:</strong> ${lastUpdatedRepo.owner.login}</p>
//               <p><strong>URL:</strong> <a href="${lastUpdatedRepo.html_url}" target="_blank">${lastUpdatedRepo.html_url}</a></p>
//             `;
//           } else {
//             repoInfoElement.innerHTML = "레포지토리를 찾을 수 없습니다.";
//           }
//         } else {
//           repoInfoElement.innerHTML = "오류 발생: GitHub API 요청에 실패했습니다.";
//         }
//       } catch (error) {
//         repoInfoElement.innerHTML = `오류 발생: ${error.message}`;
//       }
//     }
  
//     // 함수 호출
//     fetchLastUpdatedRepository();
//   });
console.log("hi")