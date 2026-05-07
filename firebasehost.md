// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyC4UxT3XFZki7anfHvKzO6ZhubhH5oj8fA",
  authDomain: "izboss.firebaseapp.com",
  projectId: "izboss",
  storageBucket: "izboss.firebasestorage.app",
  messagingSenderId: "831080603365",
  appId: "1:831080603365:web:57636832c19b0cfcb87e0c",
  measurementId: "G-W5NTRSXYJS"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);