'use client';

import { useState, useMemo } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { authApi } from '@/lib/api';

interface PasswordStrength {
  score: number;
  label: string;
  color: string;
}

export default function RegisterPage() {
  const router = useRouter();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    password: '',
    confirmPassword: '',
  });

  const passwordChecks = useMemo(() => {
    const password = formData.password;
    return {
      length: password.length >= 8,
      uppercase: /[A-Z]/.test(password),
      lowercase: /[a-z]/.test(password),
      digit: /[0-9]/.test(password),
      special: /[!@#$%^&*(),.?":{}|<>]/.test(password),
    };
  }, [formData.password]);

  const passwordStrength = useMemo((): PasswordStrength => {
    const checks = Object.values(passwordChecks);
    const passed = checks.filter(Boolean).length;

    if (passed === 0) return { score: 0, label: '', color: 'bg-gray-200' };
    if (passed <= 2) return { score: 1, label: 'Weak', color: 'bg-red-500' };
    if (passed <= 3) return { score: 2, label: 'Fair', color: 'bg-yellow-500' };
    if (passed <= 4) return { score: 3, label: 'Good', color: 'bg-blue-500' };
    return { score: 4, label: 'Strong', color: 'bg-green-500' };
  }, [passwordChecks]);

  const validatePassword = (password: string): string | null => {
    if (password.length < 8) return 'Password must be at least 8 characters';
    if (!/[A-Z]/.test(password)) return 'Password must contain at least one uppercase letter';
    if (!/[a-z]/.test(password)) return 'Password must contain at least one lowercase letter';
    if (!/[0-9]/.test(password)) return 'Password must contain at least one digit';
    if (!/[!@#$%^&*(),.?":{}|<>]/.test(password)) return 'Password must contain at least one special character';
    return null;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (formData.password !== formData.confirmPassword) {
      setError('Passwords do not match');
      return;
    }

    const passwordError = validatePassword(formData.password);
    if (passwordError) {
      setError(passwordError);
      return;
    }

    setIsLoading(true);

    try {
      const response = await authApi.register(formData.email, formData.password, formData.name);
      if (response.success) {
        router.push('/login?registered=true');
      } else {
        setError(response.message || 'Registration failed');
      }
    } catch (err) {
      setError('Registration failed. Email might already be in use.');
    } finally {
      setIsLoading(false);
    }
  };

  const CheckIcon = ({ checked }: { checked: boolean }) => (
    <svg
      className={`w-4 h-4 ${checked ? 'text-green-500' : 'text-gray-300'}`}
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      {checked ? (
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
      ) : (
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
      )}
    </svg>
  );

  return (
    <div className="min-h-screen flex">
      {/* Left Side - Branding */}
      <div className="hidden lg:flex lg:w-1/2 bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500 p-12 flex-col justify-between">
        <div>
          <h1 className="text-white text-3xl font-bold flex items-center gap-3">
            <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2" />
            </svg>
            Percival PMO
          </h1>
          <p className="text-purple-100 mt-2">Project Management Office</p>
        </div>

        <div className="space-y-6">
          <h2 className="text-white text-2xl font-semibold">Start managing projects like a pro</h2>
          <div className="space-y-4">
            <div className="flex items-center gap-3">
              <div className="p-2 bg-white/20 rounded-full">
                <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <p className="text-white/90">Unlimited projects and tasks</p>
            </div>
            <div className="flex items-center gap-3">
              <div className="p-2 bg-white/20 rounded-full">
                <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <p className="text-white/90">Team collaboration tools</p>
            </div>
            <div className="flex items-center gap-3">
              <div className="p-2 bg-white/20 rounded-full">
                <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <p className="text-white/90">Real-time progress tracking</p>
            </div>
            <div className="flex items-center gap-3">
              <div className="p-2 bg-white/20 rounded-full">
                <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <p className="text-white/90">Kanban boards and timelines</p>
            </div>
          </div>
        </div>

        <p className="text-purple-200 text-sm">
          &copy; 2024 Percival. Built with passion.
        </p>
      </div>

      {/* Right Side - Register Form */}
      <div className="flex-1 flex items-center justify-center p-8 bg-gray-50">
        <div className="w-full max-w-md">
          {/* Mobile Logo */}
          <div className="lg:hidden text-center mb-8">
            <h1 className="text-2xl font-bold text-gray-900 flex items-center justify-center gap-2">
              <svg className="w-8 h-8 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2" />
              </svg>
              Percival PMO
            </h1>
          </div>

          <div className="bg-white rounded-2xl shadow-xl p-8">
            <div className="text-center mb-6">
              <h2 className="text-2xl font-bold text-gray-900">Create your account</h2>
              <p className="text-gray-500 mt-2">Join thousands of teams using Percival</p>
            </div>

            {error && (
              <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
                <div className="flex items-center gap-2">
                  <svg className="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <p className="text-sm text-red-700">{error}</p>
                </div>
              </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4">
              <div>
                <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                  Full name
                </label>
                <input
                  id="name"
                  type="text"
                  placeholder="John Doe"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  required
                  className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none"
                />
              </div>

              <div>
                <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-1">
                  Email address
                </label>
                <input
                  id="email"
                  type="email"
                  placeholder="you@example.com"
                  value={formData.email}
                  onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                  required
                  className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none"
                />
              </div>

              <div>
                <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-1">
                  Password
                </label>
                <input
                  id="password"
                  type="password"
                  placeholder="Create a strong password"
                  value={formData.password}
                  onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                  required
                  className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none"
                />

                {/* Password Strength Indicator */}
                {formData.password && (
                  <div className="mt-3 space-y-3">
                    <div className="flex items-center gap-2">
                      <div className="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
                        <div
                          className={`h-full transition-all duration-300 ${passwordStrength.color}`}
                          style={{ width: `${(passwordStrength.score / 4) * 100}%` }}
                        />
                      </div>
                      {passwordStrength.label && (
                        <span className={`text-xs font-medium ${
                          passwordStrength.score <= 1 ? 'text-red-600' :
                          passwordStrength.score === 2 ? 'text-yellow-600' :
                          passwordStrength.score === 3 ? 'text-blue-600' : 'text-green-600'
                        }`}>
                          {passwordStrength.label}
                        </span>
                      )}
                    </div>

                    <div className="grid grid-cols-2 gap-2 text-xs">
                      <div className="flex items-center gap-1.5">
                        <CheckIcon checked={passwordChecks.length} />
                        <span className={passwordChecks.length ? 'text-green-700' : 'text-gray-500'}>
                          8+ characters
                        </span>
                      </div>
                      <div className="flex items-center gap-1.5">
                        <CheckIcon checked={passwordChecks.uppercase} />
                        <span className={passwordChecks.uppercase ? 'text-green-700' : 'text-gray-500'}>
                          Uppercase
                        </span>
                      </div>
                      <div className="flex items-center gap-1.5">
                        <CheckIcon checked={passwordChecks.lowercase} />
                        <span className={passwordChecks.lowercase ? 'text-green-700' : 'text-gray-500'}>
                          Lowercase
                        </span>
                      </div>
                      <div className="flex items-center gap-1.5">
                        <CheckIcon checked={passwordChecks.digit} />
                        <span className={passwordChecks.digit ? 'text-green-700' : 'text-gray-500'}>
                          Number
                        </span>
                      </div>
                      <div className="flex items-center gap-1.5 col-span-2">
                        <CheckIcon checked={passwordChecks.special} />
                        <span className={passwordChecks.special ? 'text-green-700' : 'text-gray-500'}>
                          Special character (!@#$%...)
                        </span>
                      </div>
                    </div>
                  </div>
                )}
              </div>

              <div>
                <label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700 mb-1">
                  Confirm password
                </label>
                <input
                  id="confirmPassword"
                  type="password"
                  placeholder="Confirm your password"
                  value={formData.confirmPassword}
                  onChange={(e) => setFormData({ ...formData, confirmPassword: e.target.value })}
                  required
                  className={`w-full px-4 py-3 border rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all outline-none ${
                    formData.confirmPassword && formData.password !== formData.confirmPassword
                      ? 'border-red-300 bg-red-50'
                      : formData.confirmPassword && formData.password === formData.confirmPassword
                      ? 'border-green-300 bg-green-50'
                      : 'border-gray-300'
                  }`}
                />
                {formData.confirmPassword && formData.password !== formData.confirmPassword && (
                  <p className="mt-1 text-xs text-red-600">Passwords do not match</p>
                )}
              </div>

              <Button
                type="submit"
                className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg transition-colors mt-2"
                isLoading={isLoading}
              >
                Create account
              </Button>
            </form>

            <div className="mt-6 text-center">
              <p className="text-gray-600">
                Already have an account?{' '}
                <Link href="/login" className="text-indigo-600 hover:text-indigo-700 font-medium">
                  Sign in
                </Link>
              </p>
            </div>
          </div>

          <p className="text-center text-gray-400 text-sm mt-8 lg:hidden">
            &copy; 2024 Percival. Built with passion.
          </p>
        </div>
      </div>
    </div>
  );
}
